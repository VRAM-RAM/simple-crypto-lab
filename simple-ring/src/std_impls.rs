//! Implements [`simple-ring`](`crate`)'s traits for standart types,for now `uN`.

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
    fn from_u128(value: u128) -> Self {
        value as u64
    }

    fn from_u64(value: u64) -> Self {
        value
    }

    fn to_u128(&self) -> u128 {
        *self as u128
    }

    fn to_u64(&self) -> u64 {
        *self
    }
}

impl ToPoly<u32> for Vec<u32> {
    fn to_poly(&self) -> crate::Polynomial<u32> {
        Polynomial::new_from_coeffs(self.clone())
    }
}

impl ToPoly<u32> for &[u32] {
    fn to_poly(&self) -> Polynomial<u32> {
        self.to_vec().to_poly()
    }
}

impl RingCompatible for u32 {
    fn from_u128(value: u128) -> Self {
        value as u32
    }

    fn from_u64(value: u64) -> Self {
        value as u32
    }

    fn to_u128(&self) -> u128 {
        *self as u128
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl ToPoly<u8> for Vec<u8> {
    fn to_poly(&self) -> crate::Polynomial<u8> {
        Polynomial::new_from_coeffs(self.clone())
    }
}

impl ToPoly<u8> for &[u8] {
    fn to_poly(&self) -> Polynomial<u8> {
        self.to_vec().to_poly()
    }
}

impl RingCompatible for u8 {

    fn from_u128(value: u128) -> Self {
        value as u8
    }

    fn from_u64(value: u64) -> Self {
        value as u8
    }

    fn to_u128(&self) -> u128 {
        *self as u128
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl ToPoly<u16> for Vec<u16> {
    fn to_poly(&self) -> crate::Polynomial<u16> {
        Polynomial::new_from_coeffs(self.clone())
    }
}

impl ToPoly<u16> for &[u16] {
    fn to_poly(&self) -> Polynomial<u16> {
        self.to_vec().to_poly()
    }
}

impl RingCompatible for u16 {
    fn from_u128(value: u128) -> Self {
        value as u16
    }

    fn from_u64(value: u64) -> Self {
        value as u16
    }

    fn to_u128(&self) -> u128 {
        *self as u128
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
