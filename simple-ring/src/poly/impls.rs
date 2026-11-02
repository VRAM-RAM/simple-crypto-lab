use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use num::{Zero};

use crate::Polynomial;

/// Implements indexing for [`Polynomial`].
/// 
/// ## Example 
/// 
/// ```rust
/// use simple_ring::Polynomial;
/// 
/// fn foo() {
///     let poly: Polynomial<u64> = Polynomial::new_from_coeffs(vec![1u64; 128]);
///     
///     // Instead of doing :
///     // /!\ Will not work because `coeffs` is private
///     // let coeff = poly.coeffs[16]; 
/// 
///     // Do :
///     let coeff = poly[16];
/// }
/// ```
impl<T, I> Index<I> for Polynomial<T>
where
    I: std::slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.coeffs[index]
    }
}

/// Implements indexing for [`Polynomial`].
/// 
/// ## Example 
/// 
/// ```rust
/// use simple_ring::Polynomial;
/// 
/// fn foo() {
///     let mut poly: Polynomial<u64> = Polynomial::new_from_coeffs(vec![1u64; 128]);
///     
///     poly[16] = 36;
/// }
/// ```
impl<T, I> IndexMut<I> for Polynomial<T>
where
    I: std::slice::SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.coeffs[index]
    }
}

/// Implements `Add` (`+`) for two [`Polynomial`]s.
///
/// ## Example
///
/// ```rust
/// use simple_ring::Polynomial;
/// 
/// pub fn foo() {
///     let poly1: Polynomial<u64> = Polynomial::new_from_coeffs(vec![1u64; 64]);
///     let poly2: Polynomial<u64> = Polynomial::new_from_coeffs(vec![2u64; 64]);
/// 
///     let sum = poly1 + poly2;
/// }
/// ```
impl<T:  Add<Output = T>> Add for Polynomial<T> 
where T: Add + Clone + Copy 
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.clone();
        sum += rhs;
        sum
    }
}   

/// Implements `Sub` (`-`) for two [`Polynomial`]s.
///
/// ## Example
///
/// ```rust
/// use simple_ring::Polynomial;
/// 
/// pub fn foo() {
///     let poly1: Polynomial<u64> = Polynomial::new_from_coeffs(vec![1u64; 64]);
///     let poly2: Polynomial<u64> = Polynomial::new_from_coeffs(vec![2u64; 64]);
/// 
///     let sum = poly1 - poly2;
/// }
/// ```
impl<T: Sub<Output = T>> Sub for Polynomial<T>
where
    T: Sub + Neg<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut diff = self.clone();
        diff -= other;
        diff
    }
}

impl<T> AddAssign for Polynomial<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        let min_len =  self.len().min(rhs.len());

        if self.len() == min_len {
            for i in min_len..rhs.len() {
                self.push(rhs[i]);
            }
        }
        for i in 0..min_len {
            self[i] = self[i] + rhs[i];
        }
    }
}


impl<T> SubAssign for Polynomial<T>
where
    T: Sub<Output = T> + Neg<Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        let min_len = self.len().min(rhs.len());

        for i in 0..min_len {
            self[i] = self[i] - rhs[i];
        }
    }
}

/// Implements the `Multiplication` of two [`Polynomial`]s.
///
/// ### Example
/// 
/// ```rust
/// use simple_ring::Polynomial;
/// 
/// pub fn foo() {
///     let poly1: Polynomial<u64> = Polynomial::new_from_coeffs(vec![1u64; 64]);
///     let poly2: Polynomial<u64> = Polynomial::new_from_coeffs(vec![2u64; 64]);
/// 
///     let prod = poly1 * poly2;
/// }
/// ```
impl<T> Mul<T> for Polynomial<T>
where
    T: MulAssign + Copy + Zero,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut prod = self.clone();
        prod *= rhs;
        prod
    }
}


impl<T> MulAssign<T> for Polynomial<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.len() {
            self[i] *= rhs;
        }
    }
}


/// Multiply (scale) a [`Polynomial`] by some value.
///
///
/// ### Example
/// ```
/// use simple_ring::Polynomial;
/// 
/// pub fn foo() {
///     let poly: Polynomial<u64> = Polynomial::new_from_coeffs(vec![1u64; 64]);
/// 
///     let scale = poly1 * 2;
/// }
/// ```
impl<T> Mul for Polynomial<T>
where
    T: Mul<Output = T> + AddAssign + Sub<Output = T>,
    T: Copy + Clone + Zero,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new = self.clone();
        new *= rhs;
        new
    }
}

impl<T> MulAssign for Polynomial<T>
where
    T: Mul<Output = T> + AddAssign + Sub<Output = T>,
    T: Copy + Clone + Zero,
{
    fn mul_assign(&mut self, rhs: Self) {
        let clone = self.clone();

        if self.len() > 0 || rhs.len() > 0 {
            self.coeffs.to_vec().resize(self.len() + rhs.len() - 1, T::zero());

            for i in 0..clone.len() {
                for j in 0..rhs.len() {
                    self[i + j] += clone[i] * rhs[j];
                }
            }
        }
    }
}

/// Divide a [`Polynomial`] by some value.
///
///
/// ```
/// use simple_ring::Polynomial;
/// 
/// pub fn foo() {
///     let poly: Polynomial<u64> = Polynomial::new_from_coeffs(vec![1u64; 64]);
/// 
///     let div = poly1 / 2;
/// }
/// ```
impl<T> Div<T> for Polynomial<T>
where
    T: DivAssign + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut prod = self.clone();
        prod /= rhs;
        prod
    }
}

impl<T> DivAssign<T> for Polynomial<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..self.len() {
            self[i] /= rhs;
        }
    }
}
