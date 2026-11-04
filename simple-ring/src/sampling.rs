//! Code for sampling of polynomials, and generation of coefficients


use crate::traits::RingCompatible;
use crate::{Polynomial, RingParams};
use rand::{Rng, RngCore, rngs::OsRng};
use shake::Shake128;
use shake::digest::{ExtendableOutput, Update, XofReader};
use std::ops::Deref;

pub enum SeedType {
    NotGiven,
    Given([u8; 32]),
}

#[derive(Clone, Debug)]
pub struct Sample(pub Vec<i32>);

impl Deref for Sample {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sample {
    pub fn to_poly_64(self, q: u64) -> Polynomial<u64> {
        let coeffs = self
            .iter()
            .map(|v| (*v as i128).rem_euclid(q as i128) as u64)
            .collect();
        Polynomial::new_from_coeffs(coeffs)
    }
}

impl Polynomial<u64> {
    pub fn random_binary(n: usize) -> Self {
        let mut coeffs = vec![0u64; n];

        for c in coeffs.iter_mut() {
            *c = OsRng.gen_range(0..=1);
        }

        Polynomial::new_from_coeffs(coeffs)
    }
}

#[inline]
/// Returns a Centered Binomial Distribution sample
///
/// This is how it works (on the left, explanations and on the right, example) :
///
/// For each coefficient, we randomly choose bits for two parts, a and b.           
///
/// `a = [0, 1, 1, 1]`           `b = [1, 0, 1, 1]`    
///
/// Then, we sum this coefficients for each part.                                   
///
/// `a = 0 + 1 + 1 + 1 = 3`      `b = 1 + 0 + 1 + 1 = 3`
///
/// Finally, the value of our coefficient is equal to a - b                         
///
/// `coeff = a - b = 3 - 3 = 0`
pub fn generate_cbd_sample(n: usize, eta: usize) -> Sample {
    let mut coeffs = vec![0i32; n]; // So first we create an empty vector which will contain the coefficients.

    let bits_per_coeff = eta; //Then we choose the number of bits for each part (a and b) -> if bit_number == 4, a and be will be the sum of 4 bits, like in the example

    let total_bits = 2 * coeffs.len() * bits_per_coeff;

    let total_bytes = (total_bits + 7) / 8;

    let mut rng_buf = vec![0u8; total_bytes];

    OsRng.fill_bytes(&mut rng_buf); //We generate a single vector of u8s of which we will choose the bits.
    let mut bit_index = 0;
    for coeff in coeffs.iter_mut() {
        let mut a = 0;
        let mut b = 0;

        for _ in 0..bits_per_coeff {
            //Then we choose the bits in the sequence of bytes generated before. (for example : generated = [00001111; 10101010; ...], then we pick the bits one by one as first bit = 0, second one = 0...)
            let byte = rng_buf[bit_index / 8];
            let bit = (byte >> (bit_index % 8)) & 1;
            a += bit as u64;
            bit_index += 1;
        }

        for _ in 0..bits_per_coeff {
            //We continue picking
            let byte = rng_buf[bit_index / 8];
            let bit = (byte >> (bit_index % 8)) & 1;
            b += bit as u64;
            bit_index += 1;
        }

        *coeff = a as i32 - b as i32; //And we finally do a - b
    }

    Sample(coeffs)
}

#[inline]
/// Generates the small-coeffs sample (`[-1; 1]`)
pub fn generate_small_sample(params: &RingParams) -> Sample {
    let mut rng = OsRng;
    let mut coeffs: Vec<i32> = Vec::with_capacity(params.n);

    for _ in 0..params.n {
        let small: i32 = rng.gen_range(-1..=1); //We generate coeffs in the alphabet A = {-1, 0, 1}
        coeffs.push(small);
    }

    Sample(coeffs)
}

#[inline]
/// Generates a polynomial with uniform distribution, in the ring
pub fn generate_uniform_polynomial<T: RingCompatible + Clone>(
    params: &RingParams,
) -> Polynomial<T> {
    let mut rng = OsRng;

    let coeffs: Vec<T> = (0..params.n)
      .map(|_| T::from_u64(rng.gen_range(0..params.q)))
      .collect();

    Polynomial::new_from_coeffs(coeffs)
}

#[inline]
/// Given a [`SeedType`] (w/ seed or without seed), returns a [`Sample`] (the hash) and its `seed`
pub fn generate_then_shake(params: &RingParams, seed_type: SeedType) -> (Sample, [u8; 32]) {
    let mut rng = OsRng;

    match seed_type {
        SeedType::Given(seed) => {
            let buf = shake_128(seed, params.n);
            let coeffs: Vec<i32> = buf.into_iter().map(|b| (b as i32) - 128).collect();

            return (Sample(coeffs), seed);
        }

        SeedType::NotGiven => {
            let mut seed = [0u8; 32];
            rng.fill_bytes(&mut seed);
            let buf = shake_128(seed, params.n);
            let coeffs: Vec<i32> = buf.into_iter().map(|b| (b as i32) - 128).collect();

            return (Sample(coeffs), seed);
        }
    }
}

/// Given a seed and the size of the `hash` you want, generates an hash using `Shake128`.  
pub fn shake_128(seed: [u8; 32], n: usize) -> Vec<u8> {
    let mut hasher = Shake128::default();
    hasher.update(&seed);
    let mut reader = hasher.finalize_xof();

    let mut buf = vec![0u8; n];
    reader.read(&mut buf);
    buf
}
