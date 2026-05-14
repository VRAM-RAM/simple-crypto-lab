#![forbid(unsafe_code)]
mod scheme;
mod config;
mod plaintext;
mod find_parameters;

pub use crate::config::BFV as BFV;
pub use crate::plaintext::BFVPlaintext;
pub use crate::find_parameters::find_valid_q as find_valid_q;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");





//test collection (yes I could have done a /test/)

#[cfg(test)]
use std::time::*;


#[test]
fn basic() {
    let bfv = BFV::for_test();
    let (public_a, public_b, secret_s) = bfv.generate_keys();

    let message = BFVPlaintext::new(
        "Bonjour, çà commence avec des accents, puis on ajoute des caractères rares : œ, æ, ß, ¥, µ. Ensuite on enchaîne avec une phrase très longue pour remplir le polynôme jusqu’à saturation. Enfin on termine avec des symboles : 0123456789 !@#$%^&*()[]{}<>/?|`~",
        &bfv
    );

    let start = Instant::now();

    let ciphertext = bfv.encrypt(&message, (&public_a, &public_b));
    let decrypted = bfv.decrypt(&ciphertext, &secret_s);

    println!("Recovered: {}", decrypted);

    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);

    assert_eq!(
        bfv.backend_decrypt(&ciphertext, &secret_s).coeffs,
        message.plain.coeffs
    );
}

#[test]
fn test_cipher_addition() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();

    let n = bfv.params.n;

    let s1 = String::from_utf8(vec![126u8; n.min(50)]).unwrap();
    let s2 = String::from_utf8(vec![126u8; n.min(50)]).unwrap();

    let pt1 = BFVPlaintext::new(&s1, &bfv);
    let pt2 = BFVPlaintext::new(&s2, &bfv);

    let ct1 = bfv.encrypt(&pt1, (&pk_a, &pk_b));
    let ct2 = bfv.encrypt(&pt2, (&pk_a, &pk_b));

    let ct_sum = bfv.sum_ciphertexts(ct1, ct2);
    let result = bfv.backend_decrypt(&ct_sum, &sk);

    for i in 0..n.min(50) {
        assert_eq!(result.coeffs[i], 252);
    }

    let s3 = String::from_utf8(vec![64u8; 50]).unwrap();
    let pt3 = BFVPlaintext::new(&s3, &bfv);
    let pt4 = BFVPlaintext::new(&s3, &bfv);

    let ct3 = bfv.encrypt(&pt3, (&pk_a, &pk_b));
    let ct4 = bfv.encrypt(&pt4, (&pk_a, &pk_b));

    let ct_sum2 = bfv.sum_ciphertexts(ct3, ct4);
    let result2 = bfv.backend_decrypt(&ct_sum2, &sk);

    for i in 0..50 {
        assert_eq!(result2.coeffs[i], 128);
    }
}

#[test]
fn test_add_cipher_plain() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();
    let n = bfv.params.n;

    let pt1 = BFVPlaintext::new_from_coeffs(vec![60u64; n], &bfv);
    let pt2 = BFVPlaintext::new_from_coeffs(vec![30u64; n], &bfv);

    let ct1 = bfv.encrypt(&pt1, (&pk_a, &pk_b));
    let result = bfv.backend_decrypt(
        &bfv.sum_ciphertext_and_plaintext(&ct1, &pt2),
        &sk
    );

    for i in 0..n.min(50) {
        assert_eq!(result.coeffs[i], 90);
    }
}

#[test]
fn test_mul_cipher_plain() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();
    let n = bfv.params.n;

    let pt1 = BFVPlaintext::new_from_coeffs(vec![10u64; n], &bfv);

    let mut scalar = vec![0u64; n];
    scalar[0] = 3;
    let pt_scalar = BFVPlaintext::new_from_coeffs(scalar, &bfv);

    let ct1 = bfv.encrypt(&pt1, (&pk_a, &pk_b));

    let result = bfv.backend_decrypt(
        &bfv.mul_ciphertext_and_plaintext(&ct1, &pt_scalar),
        &sk
    );

    for i in 0..n.min(50) {
        assert_eq!(result.coeffs[i], 30);
    }
}

#[test]
fn test_add_then_mul() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();

    let pt_a = BFVPlaintext::new_from_coeffs(vec![20u64; bfv.params.n], &bfv);
    let pt_b = BFVPlaintext::new_from_coeffs(vec![10u64; bfv.params.n], &bfv);

    let mut scalar = vec![0u64; bfv.params.n];
    scalar[0] = 3;
    let pt_s = BFVPlaintext::new_from_coeffs(scalar, &bfv);

    let ct_sum = bfv.sum_ciphertexts(
        bfv.encrypt(&pt_a, (&pk_a, &pk_b)),
        bfv.encrypt(&pt_b, (&pk_a, &pk_b)),
    );

    let ct_res = bfv.mul_ciphertext_and_plaintext(&ct_sum, &pt_s);
    let result = bfv.backend_decrypt(&ct_res, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 90);
    }
}

#[test]
fn test_mul_then_add() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();

    let pt5 = BFVPlaintext::new_from_coeffs(vec![5u64; bfv.params.n], &bfv);
    let pt7 = BFVPlaintext::new_from_coeffs(vec![7u64; bfv.params.n], &bfv);

    let mut s = vec![0u64; bfv.params.n];
    s[0] = 4;
    let pt_s = BFVPlaintext::new_from_coeffs(s, &bfv);

    let ct_mul = bfv.mul_ciphertext_and_plaintext(
        &bfv.encrypt(&pt5, (&pk_a, &pk_b)),
        &pt_s,
    );

    let ct_res = bfv.sum_ciphertexts(ct_mul, bfv.encrypt(&pt7, (&pk_a, &pk_b)));
    let result = bfv.backend_decrypt(&ct_res, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 27);
    }
}

#[test]
fn test_chained_additions() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();

    let pt = BFVPlaintext::new_from_coeffs(vec![1u64; bfv.params.n], &bfv);

    let mut acc = bfv.encrypt(&pt, (&pk_a, &pk_b));

    for _ in 1..10 {
        acc = bfv.sum_ciphertexts(
            acc,
            bfv.encrypt(&pt, (&pk_a, &pk_b))
        );
    }

    let result = bfv.backend_decrypt(&acc, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 10);
    }
}

#[test]
fn test_edge_zero() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();

    let pt_x = BFVPlaintext::new_from_coeffs(vec![128u64; bfv.params.n], &bfv);
    let pt_z = BFVPlaintext::new_from_coeffs(vec![0u64; bfv.params.n], &bfv);

    let ct = bfv.sum_ciphertexts(
        bfv.encrypt(&pt_x, (&pk_a, &pk_b)),
        bfv.encrypt(&pt_z, (&pk_a, &pk_b)),
    );

    let result = bfv.backend_decrypt(&ct, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 128);
    }
}

#[test]
fn test_edge_max_value() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();

    let pt = BFVPlaintext::new_from_coeffs(vec![255u64; bfv.params.n], &bfv);
    let ct = bfv.encrypt(&pt, (&pk_a, &pk_b));
    let result = bfv.backend_decrypt(&ct, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 255);
    }
}

#[test]
fn test_edge_add_one_to_max() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = bfv.generate_keys();

    let t = bfv.t;

    let pt_max = BFVPlaintext::new_from_coeffs(vec![t - 1; bfv.params.n], &bfv);
    let pt_one = BFVPlaintext::new_from_coeffs(vec![1u64; bfv.params.n], &bfv);

    let ct = bfv.sum_ciphertexts(
        bfv.encrypt(&pt_max, (&pk_a, &pk_b)),
        bfv.encrypt(&pt_one, (&pk_a, &pk_b)),
    );

    let result = bfv.backend_decrypt(&ct, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 0);
    }
}