use std::ops::{AddAssign, Mul};

use num::Zero;

use crate::{Polynomial, traits::RingCompatible};
#[cfg(not(feature = "parallel"))]
use crate::RingParams;

impl<T: RingCompatible + Zero + Clone + Mul<Output = T> + AddAssign + Copy> Polynomial<T> 

{
    /// Executes the **sum** of two [`Polynomial`]s, in the polynomial ring.
    /// 
    /// If the `parallel` feature is enabled, it calls `polynomial_sum_multi`. Else, it calls `polynomial_sum_single`.
    #[cfg(not(feature = "parallel"))]
    pub fn ring_sum(&self, params: &RingParams, polynomial: &Polynomial<T>) -> Polynomial<T> {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to sum doesn't match ! You may use parameters.n for both of the polynomials");

        Self::polynomial_sum_single( params,  self, polynomial)
    }


    /// Executes the **sub** of two [`Polynomial`]s, in the polynomial ring.
    /// 
    /// If the `parallel` feature is enabled, it calls `polynomial_sub_multi`. Else, it calls `polynomial_sub_single`.
    #[cfg(not(feature = "parallel"))]   
    pub fn sub(&self, params: &RingParams, polynomial: &Polynomial<T>) -> Polynomial<T> {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to substract doesn't match ! You may use parameters.n for both of the polynomials");
 
        Self::polynomial_sub_single( params,  self, polynomial)
    }

    #[cfg(feature = "parallel")]
    pub fn sum(&self, params: &RingParams, polynomial: &Polynomial<T>) -> Polynomial<T> {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to sum doesn't match ! You may use parameters.n for both of the polynomials");

        Self::polynomial_sum_multi( params,  self, polynomial)
    }

    #[cfg(feature = "parallel")]
    pub fn sub(&self, params: &RingParams, polynomial: &Polynomial<T>) -> Polynomial<T> {
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to multiply doesn't match ! You may use parameters.n for both of the polynomials");
        Self::polynomial_sub_multi( params,  self, polynomial)
    }


    #[inline]
    /// Computes a `single-thread` sum of two [`Polynomial`]s, in the ring.
    /// 
    /// `T` needs to implement [`RingCompatible`], [`Zero`] and [`Clone`].
    /// 
    /// The two polynomials should have the same length (`params.n`), and each coefficient is reduced by `q`.
    fn polynomial_sum_single(params: &RingParams, first: &Self, second: &Self) -> Self { 
        let mut coeffs = vec![T::zero(); params.n];

        for i in 0..params.n {
            coeffs[i] = T::from_u128(first.coeffs[i].to_u128() + second.coeffs[i].to_u128() % params.q as u128);
        }

        Polynomial::new_from_coeffs(coeffs)
    }

    #[inline]
     /// Computes a `single-thread` sub of two [`Polynomial`]s, in the ring.
    /// 
    /// `T` needs to implement [`RingCompatible`], [`Zero`] and [`Clone`].
    /// 
    /// The two polynomials should have the same length (`params.n`), and each coefficient is reduced by `q`.
    fn polynomial_sub_single(params: &RingParams, first: &Self, second: &Self) -> Self { 
        let mut coeffs = vec![T::zero(); params.n];

        for i in 0..params.n {
            coeffs[i] = T::from_i128((first.coeffs[i].to_i128() - second.coeffs[i].to_i128()).rem_euclid(params.q as i128));
        }

        Polynomial::new_from_coeffs(coeffs)  
    }

    #[cfg(feature = "parallel")]
    /// The multi-thread sum (experimental and not so good...)
    fn polynomial_sum_multi(params: &RingParams, first: &Self, second: &Self) -> Self { 
        let mut coeffs = vec![T::zero(); params.n];
        coeffs
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, coeff)| {
                *coeff = T::from_u128((first.coeffs[i].to_u128() + second.coeffs[i].to_u128()) % params.q.to_u128());
            });
        Self { coeffs: coeffs.into_boxed_slice() }
    }

    #[cfg(feature = "parallel")]
    /// The multi-thread substraction (experimental and not so good...)
    fn polynomial_sub_multi(params: &RingParams, first: &Self, second: &Self) -> Self { 
        let mut coeffs = vec![T::zero(); params.n];
        coeffs
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, coeff)| {
            *coeff = T::from_i128(first.coeffs[i].to_i128() - second.coeffs[i].to_i128()).rem_euclid(params.q.to_i128());
        });

        Self { coeffs: coeffs.into_boxed_slice() }
        
    }

    #[inline]
    // Function for naive multiplication in the ring, not to be used (except for small degrees)
    pub fn ring_mul(&self, params: &RingParams, polynomial: &Polynomial<T>) -> Self { 
        assert_eq!(self.coeffs.len(), polynomial.coeffs.len(), "The length of the two polynomials you're trying to multiply doesn't match ! You may use parameters.n for both of the polynomials");

        let mut b = vec![T::zero(); 2 * params.n];
        
        for i in 0..params.n {
            for j in 0..params.n {
                let k = i + j;
                b[k] += self.coeffs[i] * polynomial.coeffs[j] ;

            }
        }
        
        let mut coeffs = vec![T::zero(); params.n];

        for k in 0..params.n {
            let low = b[k];
            let high = if k + params.n < b.len() { b[k + params.n] } else { T::zero() };
            
            let val = ((low.to_i128()) - (high.to_i128())).rem_euclid(params.q as i128);
            coeffs[k] = T::from_i128(val);
        }

        Polynomial::new_from_coeffs(coeffs)
    }

    #[inline]
    /// Scales a polynomial by a constant, in the ring
    pub fn ring_scale(&self, params: &RingParams, lambda: u64) -> Self { 
        let coeffs: Vec<T> = self.coeffs
            .iter()
            .map(|&coeff| T::from_u128((coeff.to_u128() * lambda as u128) % params.q as u128))
            .collect::<Vec<T>>();
    
        Polynomial::new_from_coeffs(coeffs)
    }



    #[inline]
    /// Returns the opposite of the [`Polynomial`] ( - polynomial )
    pub fn opposite(&self, params: &RingParams) -> Self { 
        let q = params.q;
        let coeffs: Vec<T> = self.coeffs
            .iter()
            .map(|&coeff| T::from_i128(( - (coeff.to_i128())).rem_euclid(q as i128)))
            .collect::<Vec<T>>();

        Polynomial::new_from_coeffs(coeffs)
    }
    
    #[inline]
    /// Reduces a [`Polynomial`] by a constant (polynomial mod k)
    pub fn reduce(&self, constant: u128) -> Self { 
        let new = self.coeffs
            .iter()
            .map(|c| T::from_u128((c.to_u128()).rem_euclid(constant)))
            .collect();

        Polynomial::new_from_coeffs(new)
    }

    #[inline]
    /// Devides a [`Polynomial`] by a constant (polynomial / k)
    pub fn divide_by_constant( 
        &self,
        constant: u128,
    ) -> Self {
        let new = self.coeffs
            .iter()
            .map(|c| {
                T::from_u128((c.to_u128() + constant / 2) / constant)
            })
            .collect::<Vec<T>>();

        Polynomial::new_from_coeffs(new)
    }

}