
use bytemuck::Pod;
use num::Zero;
#[cfg(feature = "parallel")]
use rayon::prelude::*;          
#[cfg(feature = "parallel")]
use rayon::slice::ParallelSliceMut;
use serde::{Deserialize, Serialize};
use crate::RingParams;
use crate::ntt::{NTTprecaculated, inverse_ntt, forward_ntt};
use bytemuck::checked::cast_slice;

pub mod impls;
pub mod ring;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polynomial<T> {
    coeffs: Box<[T]>
}


impl<T : Clone> Polynomial<T> {
    /// Returns a new [`Polynomial`].
    /// 
    /// ### Example
    /// 
    /// ```rust
    /// use simple_ring::Polynomial;
    /// 
    /// fn foo() {
    ///     let poly64: Polynomial<u64> = Polynomial::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self { coeffs: vec![].into_boxed_slice() }
    }

    /// Returns a [`Polynomial`] built from the given **coefficients**.
    /// 
    /// ### Example
    /// 
    /// ```rust
    /// use simple_ring::Polynomial;
    /// 
    /// fn foo() {
    ///     let coeffs = vec![7u64; 128];
    ///     let poly64: Polynomial<u64> = Polynomial::new_from_coeffs(coeffs);
    /// }
    /// ```
    pub fn new_from_coeffs(coeffs: Vec<T>) -> Self { 
        Self { coeffs: coeffs.into_boxed_slice() }
    }

    /// Returns a [`Polynomial`] of `0` coefficients of type `C`.
    /// 
    /// `C` must implement [`Zero`] and [`Clone`].
    /// 
    /// ### Example 
    /// 
    /// ```rust
    /// use simple_ring::Polynomial;
    /// 
    /// type Poly64 = Polynomial<u64>;
    /// 
    /// fn foo() {
    ///     // Return a Polynomial with :
    ///     //  
    ///     // {0, 0, 0, ..., 0}
    ///     let zero_poly = Poly64::zeros(64);
    /// }
    /// ```
    pub fn zeros<C: Zero + Clone>(n: usize) -> Polynomial<C> { 
        Polynomial { coeffs: vec![C::zero(); n].into_boxed_slice() }
    }

    /// Appends a `value` of type `T` to the end of the given [`Polynomial`].
    /// 
    /// ### Example 
    /// 
    /// ```rust
    /// use simple_ring::Polynomial;
    /// 
    /// type Poly64 = Polynomial<u64>;
    /// 
    /// fn foo() {
    ///     // Return a Polynomial with :
    ///     //  
    ///     // {0, 0, 0, ..., 0}
    ///     let mut zero_poly = Poly64::zeros(64);
    /// 
    ///     // The polynomial becames 
    ///     //      
    ///     // {0, 0, 0, ..., 0, 1}
    ///     zero_poly.push(1); 
    /// }
    /// ```
    pub fn push(&mut self, value: T) {
        self.coeffs.to_vec().push(value);
    }


    /// Returns the len of the [`Polynomial`].
    /// 
    /// Warning : the `len` of a [`Polynomial`] is not the same than its degree. The degree is equal to `len - 1`.
    /// 
    /// For example :
    /// 
    /// `3 + 18n - 7nˆ2` is of degree 2, but of len `3`, because it has three coefficients.
    /// 
    /// ### Example 
    /// 
    /// ```rust
    /// use simple_ring::Polynomial;
    /// 
    /// type Poly64 = Polynomial<u64>;
    /// 
    /// fn foo() {
    ///     // Return a Polynomial with :
    ///     //  
    ///     // {0, 0, 0, ..., 0}
    ///     let mut zero_poly = Poly64::zeros(64);
    /// 
    ///     assert_eq!(zero_poly.len(), 64);
    /// }
    /// ```
    pub fn len(&self) -> usize {
        self.coeffs.len()
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

impl<T: Pod> Polynomial<T> {
    pub fn as_bytes(&self) -> &[u8] {
        cast_slice(&self.coeffs)
    }
}