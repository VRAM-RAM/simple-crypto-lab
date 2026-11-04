use simple_ring::{Polynomial, RingParams, find_valid_omega};

fn params() -> RingParams {
    RingParams::new(4, 17, find_valid_omega(4, 17)) // n = 4 (power of 2), q = 17 (small, fast)
}

#[test]
fn test_sub_operator() {
    let diff = Polynomial::new_from_coeffs(vec![8u64, 3, 2, 1])
        - Polynomial::new_from_coeffs(vec![2u64, 3, 2, 0]);

    // Sub is done in the T's domain (here, u64), so no reduction is applied : [8-2, 3-3, 2-2, 1-0]
    assert_eq!(diff.to_vec(), vec![6u64, 0, 0, 1]);
}

#[test]
#[should_panic]
fn test_sub_operator_underflow_panics() {
    // Like basic u64 operations, subtracting a greater value than the first one should panic
    let _diff = Polynomial::new_from_coeffs(vec![1u64, 1, 1, 1])
        - Polynomial::new_from_coeffs(vec![8u64, 8, 8, 8]);
}

#[test]
fn test_sub_assign_operator() {
    let mut diff = Polynomial::new_from_coeffs(vec![8u64, 3, 2, 1]);

    diff -= Polynomial::new_from_coeffs(vec![2u64, 3, 2, 0]);

    assert_eq!(diff.to_vec(), vec![6u64, 0, 0, 1]);
}

#[test]
fn test_ring_sub_reduces_mod_q() {
    let params = params();
    let a = Polynomial::new_from_coeffs(vec![8u64, 3, 2, 1]); // All coefficients are < q = 17
    let b = Polynomial::new_from_coeffs(vec![2u64, 15, 1, 9]);

    let diff = a.ring_sub(&params, &b);

    // The sub is done in the ring (mod q) : [8-2, 3-15, ...] % 17, with the result in [0, q)
    assert_eq!(diff.to_vec(), vec![6u64, 5, 1, 9]); // (3 - 15) % 17 = 5, (1 - 9) % 17 = 9
}
