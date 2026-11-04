use simple_ring::{Polynomial, RingParams, find_valid_omega};

fn params() -> RingParams {
    RingParams::new(4, 17, find_valid_omega(4, 17)) // n = 4 (power of 2), q = 17 (small, fast)
}

fn poly_a() -> Polynomial<u64> {
    // P1 = [1, 2, 3, 8]
    Polynomial::new_from_coeffs(vec![1u64, 2, 3, 8])
}

fn poly_b() -> Polynomial<u64> {
    // P2 = [8, 0, 1, 3]
    Polynomial::new_from_coeffs(vec![8u64, 0, 1, 3])
}

#[test]
fn test_add_operator() {
    let sum = poly_a() + poly_b();

    assert_eq!(sum.len(), 4); // The len is the number of coefficients
    assert_eq!(sum.to_vec(), vec![9u64, 2, 4, 11]); // [1+8, 2+0, 3+1, 8+3]
}

#[test]
fn test_add_assign_operator() {
    let mut sum = poly_a();
    sum += poly_b();

    assert_eq!(sum.to_vec(), vec![9u64, 2, 4, 11]);
}

#[test]
fn test_add_assign_operator_different_lengths() {
    let mut sum = Polynomial::<u64>::zeros(2); // [0, 0]
        println!("{:?}", sum);

    sum += Polynomial::new_from_coeffs(vec![1u64, 2, 3]); // [1, 2, 3]

    println!("{:?}", sum);

    assert_eq!(sum.len(), 3);
    assert_eq!(sum.to_vec(), vec![1u64, 2, 3]);
}

#[test]
fn test_ring_add_reduces_mod_q() {
    let params = params();
    let a = Polynomial::new_from_coeffs(vec![16u64, 2, 3, 8]); // All coefficients are < q = 17
    let b = Polynomial::new_from_coeffs(vec![2u64, 15, 1, 9]); // All coefficients are < q = 17

    let sum = a.ring_sum(&params, &b);

    // The sum is done in the ring (mod q) : (16 + 2) % 17 = 1, (2 + 15) % 17 = 0, ...
    assert_eq!(sum.to_vec(), vec![1u64, 0, 4, 0]);
}
