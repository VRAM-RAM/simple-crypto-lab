use num::Zero;
use std::ops::{AddAssign, Mul};

use crate::RingParams;
use crate::{Polynomial, forward_ntt, inverse_ntt, ntt::NTTprecaculated, traits::RingCompatible};

impl<T: RingCompatible + Zero + Clone + Mul<Output = T> + Copy + PartialOrd + AddAssign>
    Polynomial<T>
{
    #[inline]
    /// Computes a `single-thread` sum of two [`Polynomial`]s, in the ring.
    ///
    /// `T` needs to implement [`RingCompatible`], [`Zero`] and [`Clone`].
    ///
    /// The two polynomials should have the same length (`params.n`), and each coefficient is reduced by `q`.
    ///
    /// Warning : the coefficients should belong to [0; q). If not, the method may overflow.
    pub fn ring_sum(&self, params: &RingParams, second: &Self) -> Self {
        let q = params.q;

        Polynomial::new_from_coeffs(
            self.coeffs
                .iter()
                .zip(second.coeffs.iter())
                .map(|(a, b)| {
                    let (s, carry) = (a.to_u64()).overflowing_add(b.to_u64());
                    if carry || s >= q {
                        T::from_u64(s.wrapping_sub(q))
                    } else {
                        T::from_u64(s)
                    }
                })
                .collect(),
        )
    }

    #[inline]
    /// Computes a `single-thread` sub of two [`Polynomial`]s, in the ring.
    ///
    /// `T` needs to implement [`RingCompatible`], [`Zero`] and [`Clone`].
    ///
    /// The two polynomials should have the same length (`params.n`), and each coefficient is reduced by `q`.
    pub fn ring_sub(&self, params: &RingParams, second: &Self) -> Self {
        let q = params.q;

        Polynomial::new_from_coeffs(
            self.coeffs
                .iter()
                .zip(second.coeffs.iter())
                .map(|(a, b)| {
                    let d = a.to_u64().wrapping_sub(b.to_u64()).wrapping_add(q);

                    T::from_u64(d % q)
                })
                .collect(),
        )
    }

    #[inline]
    // Function for naive multiplication in the ring, not to be used (except for small degrees)
    pub fn ring_mul(&self, params: &RingParams, polynomial: &Polynomial<T>) -> Self {
        assert_eq!(
            self.coeffs.len(),
            polynomial.coeffs.len(),
            "The length of the two polynomials you're trying to multiply doesn't match ! You may use parameters.n for both of the polynomials"
        );

        let mut b = vec![T::zero(); 2 * params.n];

        for i in 0..params.n {
            for j in 0..params.n {
                let k = i + j;
                b[k] += self.coeffs[i] * polynomial.coeffs[j];
            }
        }

        let mut coeffs = vec![T::zero(); params.n];

        for k in 0..params.n {
            let low = b[k];
            let high = if k + params.n < b.len() {
                b[k + params.n]
            } else {
                T::zero()
            };

            let val = ((low.to_u64() as i128) - (high.to_u64() as i128))
                .rem_euclid(params.q as i128) as u64;

            coeffs[k] = T::from_u64(val);
        }

        Polynomial::new_from_coeffs(coeffs)
    }

    #[inline]
    /// Scales a polynomial by a constant, in the ring
    pub fn ring_scale(&self, params: &RingParams, lambda: u64) -> Self {
        let coeffs: Vec<T> = self
            .coeffs
            .iter()
            .map(|&coeff| T::from_u128((coeff.to_u128() * lambda as u128) % params.q as u128))
            .collect::<Vec<T>>();

        Polynomial::new_from_coeffs(coeffs)
    }

    #[inline]
    /// Returns the opposite of the [`Polynomial`] ( - polynomial )
    pub fn opposite(&self, params: &RingParams) -> Self {
        let q = params.q;
        let coeffs: Vec<T> = self
            .coeffs
            .iter()
            .map(|&coeff| {
                if coeff.is_zero() {
                    coeff
                } else {
                    T::from_u64(q.wrapping_sub(coeff.to_u64()))
                }
            })
            .collect::<Vec<T>>();

        Polynomial::new_from_coeffs(coeffs)
    }

    #[inline]
    /// Reduces a [`Polynomial`] by a constant (polynomial mod k)
    pub fn reduce(&self, constant: u128) -> Self {
        let new = self
            .coeffs
            .iter()
            .map(|c| T::from_u128((c.to_u128()).rem_euclid(constant)))
            .collect();

        Polynomial::new_from_coeffs(new)
    }

    #[inline]
    /// Devides a [`Polynomial`] by a constant (polynomial / k)
    pub fn divide_by_constant(&self, constant: u128) -> Self {
        let new = self
            .coeffs
            .iter()
            .map(|c| T::from_u128((c.to_u128() + constant / 2) / constant))
            .collect::<Vec<T>>();

        Polynomial::new_from_coeffs(new)
    }

    #[inline]
    /// Computes the product of two polynomials by passing them in NTTs.
    pub fn mul_ntt(
        &self,
        params: &RingParams,
        ntt_tables: &NTTprecaculated,
        polynomial: &Self,
    ) -> Self {
        assert_eq!(
            self.coeffs.len(),
            polynomial.coeffs.len(),
            "The length of the two polynomials you're trying to multiply doesn't match ! You may use parameters.n for both of the polynomials"
        );
        let a = forward_ntt(params, self, ntt_tables);
        let b = forward_ntt(params, polynomial, ntt_tables);

        let c_ntt = a.ring_pointwise_mul(params, &b);

        inverse_ntt(params, &c_ntt, ntt_tables)
    }

    #[inline]
    /// Computes the pointwise mul of two Polynomials (a ◦ b)
    pub fn ring_pointwise_mul(&self, params: &RingParams, polynomial: &Self) -> Self {
        let mut result = vec![T::zero(); params.n];

        for i in 0..params.n {
            result[i] =
                T::from_u128((self[i].to_u128() * polynomial[i].to_u128()) % params.q as u128);
        }

        Polynomial::new_from_coeffs(result)
    }
}
