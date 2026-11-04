use crate::Polynomial;

/// Trait to implement for your custom type, to make it compatible with operations in the `ring`, such
/// as the NTT.
pub trait RingCompatible {
    fn to_u128(&self) -> u128;
    fn to_u64(&self) -> u64;

    fn from_u64(value: u64) -> Self;
    fn from_u128(value: u128) -> Self;
}

/// Traits to implement for your custom type, to make it compatible with the `to_poly()` method.
///
/// ## Example
///
/// ```rust
/// use simple_ring::{Polynomial, ToPoly};
///
/// fn foo() {
///     let coeffs = vec![7u64; 12];
///
///     // Polynomial<u64>
///     let poly = coeffs.to_poly();
/// }
/// ```
pub trait ToPoly<T: Clone> {
    fn to_poly(&self) -> Polynomial<T>;
}
