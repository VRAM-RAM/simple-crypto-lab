pub mod ring;
pub mod ntt;
pub mod poly;
pub mod modular;
pub mod sampling;
pub mod encoding;
pub mod bitwriting;
pub mod traits;
pub mod std_impls;

pub use ring::RingParams as RingParams;
pub use poly::Polynomial as Polynomial;
pub use traits::ToPoly;
pub use modular::{find_valid_omega, is_q_valid, mod_pow, is_prime};
pub use sampling::{generate_small_sample, generate_cbd_sample, generate_uniform_polynomial, generate_then_shake, shake_128, Sample, SeedType};
pub use ntt::{forward_ntt, inverse_ntt, precalculate};
pub use bitwriting::BitWriter;

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
    coeffs[3] = (-8 as i64).rem_euclid(params.q as i64) as u64;//We have to reduce because the opposite is done modulo q
    assert_eq!(coeffs, opposite.to_vec()); 
    println!()
}