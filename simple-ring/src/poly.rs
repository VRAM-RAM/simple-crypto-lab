//! Defines the [`Polynomial`] structure, and basic methods.
//!
//! Also implements `as_bytes()` from [**bytemuck**](`bytemuck`)
//!
//! In lower modules ([`impls`] and [`ring`]), it defines the operations (sum, sub, mul...) and operations in the rings (ntt mul, ring's sum...)

use std::ops::Mul;

use bytemuck::Pod;
use bytemuck::checked::cast_slice;
use num::Zero;


use serde::{Deserialize, Serialize};

pub mod impls;
pub mod ring;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polynomial<T> {
    coeffs: Box<[T]>,
}

#[macro_export]
macro_rules! poly {
    ($($args:tt)*) => (
         $crate::Polynomial::new_from_coeffs(vec![$($args)*])
    );
}

impl<T: Clone> Polynomial<T> {
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
        Self {
            coeffs: vec![].into_boxed_slice(),
        }
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
        Self {
            coeffs: coeffs.into_boxed_slice(),
        }
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
    ///     let zero_poly: Polynomial<u64> = Poly64::zeros(64);
    /// }
    /// ```
    pub fn zeros<C: Zero + Clone>(n: usize) -> Polynomial<C> {
        Polynomial {
            coeffs: vec![C::zero(); n].into_boxed_slice(),
        }
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
    ///     let mut zero_poly: Polynomial<u64> = Poly64::zeros(64);
    ///
    ///     // The polynomial becames
    ///     //      
    ///     // {0, 0, 0, ..., 0, 1}
    ///     zero_poly.push(1);
    /// }
    /// ```
    pub fn push(&mut self, value: T) {
        let mut coeffs = self.coeffs.to_vec();
        coeffs.push(value);
        self.coeffs = coeffs.into_boxed_slice();
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
    ///     let mut zero_poly: Polynomial<u8> = Poly64::zeros(64);
    ///
    ///     assert_eq!(zero_poly.len(), 64);
    /// }
    /// ```
    pub fn len(&self) -> usize {
        self.coeffs.len()
    }

    /// Converts the [`Polynomial`] into a [`Vec`]
    ///
    /// ### Example
    ///
    /// ```rust
    /// use simple_ring::Polynomial;
    ///
    /// fn foo() {
    ///
    ///     let poly: Polynomial<u16> = Polynomial::new_from_coeffs(vec![8u16; 13]);
    ///
    ///     let vec: Vec<u16> = poly.to_vec();   
    /// }
    /// ```
    pub fn to_vec(&self) -> Vec<T> {
        self.coeffs.to_vec()
    }



}
 
impl<T: Mul<Output = T> + Zero + Clone> Polynomial<T> {
    /// Evaluates the [`Polynomial`] at a point `x`.
    /// 
    /// ### Example
    /// 
    /// ```rust
    /// use simple_ring::*;
    /// 
    /// fn foo() {
    ///     let poly: Polynomial<i16> = poly![-1, 3, 17, 9, 3];
    /// 
    ///     let poly_5: i16 = poly.eval(5);
    /// 
    ///     assert_eq!(poly_5, 3439)
    /// }
    /// ```
    pub fn eval(&self, x: T) -> T {
        let mut result = T::zero();

        for coeff in self.coeffs.iter().rev() {
            result =  coeff.clone() + result * x.clone()
        }

        result
    }
}

impl<T: Pod> Polynomial<T> {
    pub fn as_bytes(&self) -> &[u8] {
        cast_slice(&self.coeffs)
    }
}
