//! Implements [`simple-ring`](`crate`)'s traits for standart types, such as `u64`, `u8`, `i128`...

use crate::{Polynomial, ToPoly, traits::RingCompatible};

impl ToPoly<u64> for Vec<u64> {
    fn to_poly(&self) -> crate::Polynomial<u64> {
        Polynomial::new_from_coeffs(self.clone())
    }
}

impl ToPoly<u64> for &[u64] {
    fn to_poly(&self) -> Polynomial<u64> {
        self.to_vec().to_poly()
    }
}

impl RingCompatible for u64 {
    fn from_i128(value: i128) -> Self {
        value as u64
    }

    fn from_u128(value: u128) -> Self {
        value as u64
    }

    fn from_u64(value: u64) -> Self {
        value
    }

    fn to_i128(&self) -> i128 {
        *self as i128
    }

    fn to_u128(&self) -> u128 {
        *self as u128
    }

    fn to_u64(&self) -> u64 {
        *self
    }
}
