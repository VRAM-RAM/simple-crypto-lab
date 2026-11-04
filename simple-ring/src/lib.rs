//! # `simple-ring` : Simple implementation of mathematical primitives for LWE-based crypto in Rust
//!
//! `simple-ring` provides mathematical primitives, such as NTT or Polynomials, but also coefficient sampling for LWE-based schemes. For more informations and explanations, please see `/docs/simple-ring.pdf`.
//!
//! ## Warning
//!
//! This code isn’t audited, isn’t intended for production use, and is vulnerable to timing-based attacks.
//!
//! ## Quick start
//!
//! To use it, simply do :
//! ```bash
//! cargo add simple-ring
//! ```
//!
//!
//! ### Examples 
//!
//! #### Using the 'basic' polynomials
//! 
//! You can use `simple-ring` to manipulate **Polynomials** as wanted. For example :
//!
//! ```rust
//! use simple_ring::*;
//!
//! fn main() {
//!     let poly_p = poly![-5, 3, 2];
//!     let poly_q = poly![2, -4, 1];
//!
//!     println!("P(2) = {}", poly_p.eval(2));
//! 
//!     println!("Q(-1) = {}", poly_q.eval(-1));
//!
//!     println!("A(x) = P(x) + Q(x) = {:?}", poly_p.clone() + poly_q.clone());
//!
//!     println!("B(x) = P(x) - Q(x) = {:?}", poly_p.clone() - poly_q);
//! 
//!     println!("C(x) = P(x) * 10 = {:?}", poly_p * 10);
//! }
//! ```
//!
//! It displays :
//!
//! ```bash
//! P(2) = 9
//! Q(-1) = 7
//! A(x) = P(x) + Q(x) = Polynomial { coeffs: [-3, -1, 3] }
//! B(x) = P(x) - Q(x) = Polynomial { coeffs: [-7, 7, 1] }
//! C(x) = P(x) * 10 = Polynomial { coeffs: [-50, 30, 20] }
//! ```
//!
//! #### Using the NTT multiplication and computation in the ring
//!
//! But `simple-ring` also provides a specific codebase for manipulating **Polynomials** in **ℤ𝑞 [𝑋]/(𝑋𝑛 + 1)**, including the NTT. For example :
//!
//! ```rust
//! use std::time::Instant;
//!
//! use simple_ring::*;
//!
//! fn main() {
//!     // Define the ring 
//!     let n = 4096;
//!
//!     // Yep, this is huge. To validate q, you can use `is_q_valid()`
//!     let q = 512032769;
//!
//!     let params = &RingParams::new(n, q, find_valid_omega(n, q) );
//!
//!     // Precompute the NTT tables 
//!     let ntt_tables = &precalculate(params);
//!     
//!     // Generate a polynomial with a random definition, in the ring
//!     let poly_1 = generate_uniform_polynomial::<u64>(params);
//!
//!    
//!     // Create a polynomial of `2`
//!     let poly_2 = Polynomial::new_from_coeffs(vec![2u64; n]);
//!
//!
//!     // Compute naively the product of poly_1 and poly_2
//!
//!     let start = Instant::now();
//!
//!     let product = poly_1.ring_mul(params, &poly_2);
//! 
//!     let end = Instant::now();
//!
//!     let elapsed = end - start;
//!
//!     println!("Computed P1(x) * P2(x) in {} ms with naive mul", elapsed.as_millis());
//!
//!     // Compute using ntt
//!
//!     let start = Instant::now();
//!
//!     let product_with_ntt = poly_1.mul_ntt(params, ntt_tables, &poly_2);
//!
//!     let end = Instant::now();
//!
//!     let elapsed = end - start;
//! 
//!     println!("Computed P1(x) * P2(x) in {} ms with NTT mul", elapsed.as_millis());
//!
//!     assert_eq!(product.to_vec(), product_with_ntt.to_vec());
//! }
//! ```
//! 
//! And it displays : 
//! 
//! ```bash
//! Computed P1(x) * P2(x) in 195 ms with naive mul
//! Computed P1(x) * P2(x) in 2 ms with NTT mul
//! ```
//!
//! > NTT really makes the difference on huge products
//!
//! #### Using directly the NTT, and built-in functions 
//!
//! ```rust
//! use std::time::Instant;
//!
//! use simple_ring::{RingParams, find_valid_omega, forward_ntt, inverse_ntt, mod_pow, precalculate, sampling::generate_cbd_sample};
//!
//! fn main() {
//!     // Define the ring 
//!     let n = 4096;
//!
//!     // Yep, this is huge. To validate q, you can use `is_q_valid()`
//!     let q = 512032769;
//!
//!     let params = &RingParams::new(n, q, find_valid_omega(n, q) );
//!
//!     // Precompute the NTT tables 
//!     let ntt_tables = &precalculate(params);
//!     
//!     // A sample generated using `Centered Binomial Distribution`
//!     let cbd_sample = generate_cbd_sample(n, 8);
//!
//!     // We pass the cbd_sample in the ring
//!     let poly = cbd_sample.to_poly_64(q);
//!
//!     // We convert the polynomial in NTT domain
//!     let poly_in_ntt_domain = forward_ntt(params, &poly, ntt_tables);
//!
//!     assert!(poly.to_vec() != poly_in_ntt_domain.to_vec());
//!
//!     // We pass back in the ring ('normal' domain)
//!     let poly_back_in_ring = inverse_ntt(params, &poly_in_ntt_domain, ntt_tables);
//!
//!     // Coefficients are the same
//!     assert_eq!(poly.to_vec(), poly_back_in_ring.to_vec());
//!
//!     // I don't know why, but I want to compute `1546^8889 mod q`
//!     //
//!     // Doing it naively would be to heavy...
//!
//!     let start = Instant::now();
//!
//!     let result = mod_pow(1546, 8889, q as u128);
//!
//!     let end = Instant::now();
//!
//!     let elapsed = end - start;
//!
//!     println!("1546^8889 mod {} = {} ; Computed in {} ns.", q, result, elapsed.as_nanos())
//! }
//! ```
//! 
//! And it displays :
//!
//! ```bash
//! 1546^8889 mod 512032769 = 220811177 ; Computed in 240 ns.
//! ```
//!
//! ## Tests & modifications
//!
//! To run the tests, or modify the code, you can git clone the repo, and then run the tests or do any modification you want :
//! ```bash
//! cargo test --release -- --nocapture
//! ```
//!
//! ## Repository
//!
//! `simple-ring` is a crate from [`simple-crypto-lab`](https://github.com/VRAM-RAM/simple-crypto-lab).
//!
//! ## License
//!
//! This code is licensed under the same license as `simple-crypto-lab`, so under CECILL-B or APACHE 2.0 License, your choice.
//!

pub mod bitwriting;
pub mod encoding;
pub mod modular;
pub mod ntt;
pub mod poly;
pub mod ring;
pub mod sampling;
pub mod std_impls;
pub mod traits;

pub use bitwriting::BitWriter;
pub use modular::{find_valid_omega, is_prime, is_q_valid, mod_pow};
pub use ntt::{forward_ntt, inverse_ntt, precalculate};
pub use poly::Polynomial;
pub use ring::RingParams;
pub use sampling::{
    Sample, SeedType, generate_cbd_sample, generate_small_sample, generate_then_shake,
    generate_uniform_polynomial, shake_128,
};
pub use traits::ToPoly;

#[cfg(test)]
#[test]
fn test_polynomials() {
    let params = RingParams::new(4, 17, find_valid_omega(4, 17)); //We define the parameters
    let ntt_tables = &precalculate(&params);
    let mut coeffs = vec![0u64; 4]; //We create the coefficients for our first polynomial
    coeffs[3] = 8;
    let poly1 = Polynomial::new_from_coeffs(coeffs.clone()); //We create the first polynomial as P1 = [0, 0, 0, 8]
    let poly2 = Polynomial::<u64>::zeros(4); //We create an empty polynomial, which will be the second one.
    let sum = poly1.ring_sum(&params, &poly2); //We execute the defined methods 
    let mul = poly1.ring_mul(&params, &poly2);
    let mul_ntt = poly1.mul_ntt(&params, ntt_tables, &poly2);
    let scaled = poly1.ring_scale(&params, 10);
    let divided = poly1.divide_by_constant(2);
    let reduced = poly1.reduce(2);
    let opposite = poly1.opposite(&params);
    println!();
    assert_eq!(poly1.to_vec(), coeffs.to_vec()); //We ensure, with all the assert_eqs, that the result is correct.
    println!("First polynomial : {:?}", poly1);
    println!();
    println!("Second polynomial : {:?}", poly2);
    assert_eq!(poly2.to_vec(), vec![0u64; 4]);
    println!();
    println!("Sum is : {:?}", sum);
    assert_eq!(poly1.to_vec(), sum.to_vec());
    println!();
    println!("Product is : {:?}", mul);
    assert_eq!(poly2.to_vec(), mul.to_vec());
    println!();
    println!("Product with NTT is : {:?}", mul_ntt);
    assert_eq!(poly2.to_vec(), mul.to_vec());
    println!();
    println!("Scaled first polynomial is : {:?}", scaled);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = (8 * 10) % params.q; //We have to reduce because the scale is done modulo q
    assert_eq!(coeffs, scaled.to_vec());
    println!();
    println!("Divided first polynomial is : {:?}", divided);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = 8 / 2;
    assert_eq!(coeffs, divided.to_vec());
    println!();
    println!("Reduced first polynomial is : {:?}", reduced);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = 8 % 2;
    assert_eq!(coeffs, reduced.to_vec());
    println!();
    println!("Opposite poly1 is : {:?}", opposite);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = (-8 as i64).rem_euclid(params.q as i64) as u64; //We have to reduce because the opposite is done modulo q
    assert_eq!(coeffs, opposite.to_vec());
    println!()
}
