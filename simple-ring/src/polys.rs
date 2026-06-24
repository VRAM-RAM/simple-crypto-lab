
use rand::RngCore;
use rand::rngs::OsRng;
#[cfg(feature = "parallel")]
use rayon::prelude::*;          
#[cfg(feature = "parallel")]
use rayon::slice::ParallelSliceMut;
use crate::RingParams;
use crate::ntt::{NTTprecaculated, inverse_ntt, forward_ntt};
use bytemuck::checked::cast_slice;


#[derive(Debug, Clone)]
pub struct Polynomial { //The polynomial struct, one of the bricks of the project.
    pub coeffs: Box<[u64]>,    
}

#[allow(dead_code)]
pub trait ToPoly {
    fn to_poly(&self) -> Polynomial;
}

impl<'a> ToPoly for &'a [u8] {  
    fn to_poly(&self) -> Polynomial {
        let coeffs: Vec<u64> = self
            .chunks(8)
            .map(|chunk| {
                let mut bytes = [0u8; 8];
                bytes[..chunk.len()].copy_from_slice(chunk);

                u64::from_le_bytes(bytes)
            })
            .collect();
        Polynomial::new(coeffs)
    }
}

impl Polynomial {
    pub fn new(coeffs: Vec<u64>) -> Self { //Creates, from existing coefficients, a Polynomial.
        Self { coeffs: coeffs.into_boxed_slice() }
    }

    pub fn random(n: usize) -> Self {
        let mut coeffs = vec![0u64; n];
        for c in coeffs.iter_mut() {
        *c = OsRng.next_u64() & 1;
        }
        Polynomial::new(coeffs)
    }

    pub fn zeros(n: usize) -> Self { //Creates an empty Polynomial.
        Self { coeffs: vec![0u64; n].into_boxed_slice() }
    }

    pub fn as_bytes(&self) -> &[u8] {
        cast_slice(&self.coeffs)
    }

    //Code for calling single or parallel polynomial code


    #[cfg(not(feature = "parallel"))]
    pub fn sum(&self, params: &RingParams, polynomial: &Polynomial) -> Polynomial {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to sum doesn't match ! You may use parameters.n for both of the polynomials");

        Self::polynomial_sum_single( params,  self, polynomial)
    }

    #[cfg(not(feature = "parallel"))]   
    pub fn sub(&self, params: &RingParams, polynomial: &Polynomial) -> Polynomial {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to substract doesn't match ! You may use parameters.n for both of the polynomials");
 
        Self::polynomial_sub_single( params,  self, polynomial)
    }

    #[cfg(feature = "parallel")]
    pub fn sum(&self, params: &RingParams, polynomial: &Polynomial) -> Polynomial {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to sum doesn't match ! You may use parameters.n for both of the polynomials");

        Self::polynomial_sum_multi( params,  self, polynomial)
    }

    #[cfg(feature = "parallel")]
    pub fn sub(&self, params: &RingParams, polynomial: &Polynomial) -> Polynomial {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to multiply doesn't match ! You may use parameters.n for both of the polynomials");
        Self::polynomial_sub_multi( params,  self, polynomial)
    }


    #[inline]
    pub fn mul(&self, params: &RingParams, polynomial: &Polynomial) -> Self { //Function for naive multiplication, not to be used
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to multiply doesn't match ! You may use parameters.n for both of the polynomials");

        let mut b = vec![0u64; 2 * params.n];
        
        for i in 0..params.n {
            for j in 0..params.n {
                let k = i + j;
                b[k] += self.coeffs[i]  * polynomial.coeffs[j] ;

            }
        }
        
        let mut coeffs = vec![0u64; params.n];
        for k in 0..params.n {
            let low = b[k];
            let high = if k + params.n < b.len() { b[k + params.n] } else { 0 };
            
            let val = ((low as i128) - (high as i128)).rem_euclid(params.q as i128) as u64;
            coeffs[k] = val;
        }

        Polynomial::new(coeffs)
    }

    #[inline]
    fn polynomial_sum_single(params: &RingParams, first: &Self, second: &Self) -> Self { //The single-thread sum.
        let mut coeffs = vec![0u64; params.n];
        for i in 0..params.n {
            coeffs[i] = ((first.coeffs[i] as u128 + second.coeffs[i] as u128) % params.q as u128) as u64;
        }
        Polynomial::new(coeffs)
    }

    #[inline]
    fn polynomial_sub_single(params: &RingParams, first: &Self, second: &Self) -> Self { //The single-thread substraction.
        let mut coeffs = vec![0u64; params.n];
        for i in 0..params.n {
            coeffs[i] = ((first.coeffs[i] as i128 - second.coeffs[i] as i128).rem_euclid(params.q as i128)) as u64 ;
        }
        Polynomial::new(coeffs)  
    }

    #[cfg(feature = "parallel")]
    fn polynomial_sum_multi(params: &RingParams, first: &Self, second: &Self) -> Self { //The multi-thread sum (experimental and not so good...)
        let mut coeffs = vec![0u64; params.n];
        coeffs
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, coeff)| {
                *coeff = ((first.coeffs[i] as u128 + second.coeffs[i] as u128) % params.q as u128) as u64;
            });
        Self { coeffs: coeffs.into_boxed_slice() }
    }

    #[cfg(feature = "parallel")]
    fn polynomial_sub_multi(params: &RingParams, first: &Self, second: &Self) -> Self { //The multi-thread substraction (experimental and not so good...)
        let mut coeffs = vec![0u64; params.n];
        coeffs
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, coeff)| {
            *coeff = (first.coeffs[i] as i128 - second.coeffs[i] as i128).rem_euclid(params.q as i128) as u64;
        });

        Self { coeffs: coeffs.into_boxed_slice() }
        
    }
    

    #[inline]
    pub fn scale(&self, params: &RingParams, lambda: u64) -> Self { //Function to scale a polynomial by a constant.
        let coeffs: Vec<u64> = self.coeffs
            .iter()
            .map(|&coeff| ((coeff as u128 * lambda as u128) % params.q as u128) as u64)
            .collect::<Vec<u64>>();
    
        Polynomial::new(coeffs)
    }

    #[inline]
    pub fn opposite(&self, params: &RingParams) -> Self { //Function that gives the opposite of the polynomial ( - polynomial )
        let q = params.q;
        let coeffs: Vec<u64> = self.coeffs
            .iter()
            .map(|&coeff| ( - (coeff as i128 )).rem_euclid(q as i128) as u64)
            .collect::<Vec<u64>>();
        Polynomial::new(coeffs)
    }
    
    #[inline]
    pub fn reduce(&self, constant: u128) -> Polynomial { //Function that reduce the polynomial by a constant (polynomial mod k)
        let new = self.coeffs
            .iter()
            .map(|c| (*c as u128).rem_euclid(constant) as u64)
            .collect();
        Polynomial::new(new)
    }

    #[inline]
    pub fn divide_by_constant( //Function that devide by a constant (polynomial / k)
        &self,
        constant: u128,
    ) -> Polynomial {

        let new = self.coeffs
            .iter()
            .map(|c| {
                let x = (*c as u128 + constant / 2) / constant;
                x as u64
            })
            .collect();

        Polynomial::new(new)
    }

    #[inline]
    pub fn mul_ntt(&self, params: &RingParams, ntt_tables: &NTTprecaculated, polynomial: &Polynomial) -> Polynomial { //Function that computes the product of two polynomials by passing them in NTTs.
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to multiply doesn't match ! You may use parameters.n for both of the polynomials");
        let a = forward_ntt(params, self, ntt_tables);
        let b = forward_ntt(params, polynomial, ntt_tables);

        let c_ntt = a.pointwise_mul(params, &b);

        inverse_ntt(params, &c_ntt, ntt_tables)
        
    }

    #[inline]
    pub fn pointwise_mul(&self, params: &RingParams, polynomial: &Polynomial) -> Polynomial { //Function that computes the pointwise mul of two Polynomials (a ◦ b)
        let mut result = vec![0u64; params.n];
        for i in 0..params.n {
            result[i] = ((self.coeffs[i] as u128 * polynomial.coeffs[i] as u128) % params.q as u128) as u64;
        }
        Polynomial::new(result)
    }
}