//Code for sampling of polynomials, and generation of coefficients
use crate::{Polynomial, RingParams};
use rand::{Rng, RngCore, rngs::OsRng};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Noise(pub Vec<i32>);

impl Deref for Noise {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Noise {
    pub fn to_poly(self, q:u64) -> Polynomial {
        let coeffs = self.iter().map(|v| (*v).rem_euclid(q as i32) as u64).collect();
        Polynomial::new(coeffs)
    }
}



#[inline]
pub fn generate_cbd_noise(n: usize, eta: usize) -> Noise { //Inter function to create Centered Binomial Distribution

    /*
    This is how it works (on the left, explanations and on the right, example) :
    For each coefficient, we randomly choose bits for two parts, a and b.           | a = [0, 1, 1, 1]           b = [1, 0, 1, 1]    
    Then, we sum this coefficients for each part.                                   | a = 0 + 1 + 1 + 1 = 3      b = 1 + 0 + 1 + 1 = 3
    Finally, the value of our coefficient is equal to a - b                         | coeff = a - b = 3 - 3 = 0
    */

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

        for _ in 0..bits_per_coeff { //Then we choose the bits in the sequence of bytes generated before. (for example : generated = [00001111; 10101010; ...], then we pick the bits one by one as first bit = 0, second one = 0...)
            let byte = rng_buf[bit_index / 8];
            let bit = (byte >> (bit_index % 8)) & 1; 
            a += bit as u64;
            bit_index += 1;
        }

        for _ in 0..bits_per_coeff { //We continue picking
            let byte = rng_buf[bit_index / 8];
            let bit = (byte >> (bit_index % 8)) & 1;
            b += bit as u64;
            bit_index += 1;
        }

        *coeff = a as i32 - b as i32; //And we finally do a - b mod q
    }
    
    Noise(coeffs)
}


#[inline]
pub fn generate_small_polynomial(params: &RingParams) -> Polynomial { //Intern function that generates the small-coeffs polynomials
    let mut rng = OsRng; 
    let mut coeffs = Vec::with_capacity(params.n);
    
    for _ in 0..params.n {
        let small: i8 = rng.gen_range(-1..=1); //We generate coeffs in the alphabet A = {-1, 0, 1}
        let val = if small == -1 { params.q - 1 } else { small as u64 };
        coeffs.push(val);
    }
    
    Polynomial { coeffs: coeffs.into_boxed_slice() } //We finally create the polynomial
}

#[inline]
pub fn generate_uniform_polynomial(params: &RingParams) -> Polynomial { //Intern function for generating a polynomial with uniform distribution
    let mut rng = OsRng;
    let mut coeffs = Vec::with_capacity(params.n);

    for _ in 0..params.n {
        let uniform: u64 = rng.gen_range(0..=params.q - 1); // We generate values that are in the ring 
        coeffs.push(uniform);
    }

    Polynomial { coeffs: coeffs.into_boxed_slice() }
}