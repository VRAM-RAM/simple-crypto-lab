pub mod ring;
pub mod ntt;
pub mod polys;
pub mod modular;
pub mod sampling;
pub mod encoding;
pub mod bitwriting;
pub use ring::RingParams as RingParams;
pub use polys::Polynomial as Polynomial;
pub use polys::ToPoly;
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
    let poly1 = Polynomial::new(coeffs.clone()); //We create the first polynomial as P1 = [0, 0, 0, 8]
    let poly2 = Polynomial::zeros(4); //We create an empty polynomial, which will be the second one.
    let sum = poly1.sum(&params, &poly2); //We execute the defined methods 
    let mul = poly1.mul(&params, &poly2);
    let mul_ntt = poly1.mul_ntt(&params, ntt_tables, &poly2); 
    let scaled = poly1.scale(&params, 10);
    let divided = poly1.divide_by_constant(2);
    let reduced = poly1.reduce(2);
    let opposite = poly1.opposite(&params);
    println!();
    assert_eq!(poly1.coeffs, coeffs.into_boxed_slice()); //We ensure, with all the assert_eqs, that the result is correct.
    println!("First polynomial : {:?}", poly1);
    println!();
    println!("Second polynomial : {:?}", poly2);
    assert_eq!(poly2.coeffs, vec![0u64; 4].into_boxed_slice());
    println!();
    println!("Sum is : {:?}", sum);
    assert_eq!(poly1.coeffs, sum.coeffs);
    println!();
    println!("Product is : {:?}", mul);
    assert_eq!(poly2.coeffs, mul.coeffs);
    println!();
    println!("Product with NTT is : {:?}", mul_ntt);
    assert_eq!(poly2.coeffs, mul.coeffs);
    println!();
    println!("Scaled first polynomial is : {:?}", scaled);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = (8 * 10) % params.q; //We have to reduce because the scale is done modulo q
    assert_eq!(coeffs.into_boxed_slice(), scaled.coeffs);
    println!();
    println!("Divided first polynomial is : {:?}", divided);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = 8 / 2;
    assert_eq!(coeffs.into_boxed_slice(), divided.coeffs);
    println!();
    println!("Reduced first polynomial is : {:?}", reduced);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = 8 % 2;
    assert_eq!(coeffs.into_boxed_slice(), reduced.coeffs);
    println!();
    println!("Opposite poly1 is : {:?}", opposite);
    let mut coeffs = vec![0u64; 4];
    coeffs[3] = (-8 as i64).rem_euclid(params.q as i64) as u64;//We have to reduce because the opposite is done modulo q
    assert_eq!(coeffs.into_boxed_slice(), opposite.coeffs); 
    println!()
}