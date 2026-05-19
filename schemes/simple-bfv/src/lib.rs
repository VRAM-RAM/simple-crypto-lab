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




/*
Tests for simple-bfv (yes I could have done a /tests)
*/
#[cfg(test)]
use std::time::*;

#[cfg(test)]
//For separated key generation : a single function, so that it isn't to loud in the tests
fn generate_keys(bfv: &BFV) -> (simple_ring::Polynomial, simple_ring::Polynomial, simple_ring::Polynomial) {
    let s = bfv.generate_secret_key();
    let a = bfv.generate_public_a();
    let b = bfv.generate_public_b(&a, &s);
    (a, b, s)
}

#[cfg(test)]
//For asserting a plaintext and it's decoded one
fn assert_plaintext_eq(got: &simple_ring::Polynomial, expected: &simple_ring::Polynomial, n_check: usize) {
    for i in 0..n_check.min(got.coeffs.len()) {
        assert_eq!(
            got.coeffs[i], expected.coeffs[i],
            "Coefficient mismatch at index {}: got {}, expected {}",
            i, got.coeffs[i], expected.coeffs[i]
        );
    }
}


#[test]
//Basic test for only encryption + decryption
fn basic() {
    let bfv = BFV::for_medium(); 
    let (public_a, public_b, secret_s) = generate_keys(&bfv);

    let message = BFVPlaintext::new(
        "Hello, this is BFV scheme. This basic test is here to ensure that we can encrypt special characters, as #{[|@*$ù%§! and others, like µ~'</&œ. If you want to see it totally, use for_large or for_medium.",
        &bfv
    );

    let start = Instant::now();

    let ciphertext = bfv.encrypt(&message, &public_a, &public_b);
    let decrypted = bfv.decrypt(&ciphertext, &secret_s);

    println!("Recovered: {}", decrypted);
    println!("Elapsed: {:?}", start.elapsed());

    assert_eq!(
        bfv.backend_decrypt(&ciphertext, &secret_s).coeffs,
        message.plain.coeffs,
        "Decryption mismatch"
    );
}


//Homomorphic tests

#[test]
fn test_cipher_addition() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let s1 = String::from_utf8(vec![126u8; n.min(50)]).unwrap();
    let s2 = String::from_utf8(vec![126u8; n.min(50)]).unwrap();

    let pt1 = BFVPlaintext::new(&s1, &bfv);
    let pt2 = BFVPlaintext::new(&s2, &bfv);

    let ct1 = bfv.encrypt(&pt1, &pk_a, &pk_b);
    let ct2 = bfv.encrypt(&pt2, &pk_a, &pk_b);

    let ct_sum = bfv.sum_ciphertexts(ct1, ct2);
    let result = bfv.backend_decrypt(&ct_sum, &sk);

    for i in 0..n.min(50) {
        assert_eq!(result.coeffs[i], 252, "Addition failed at index {}", i);
    }

    let s3 = String::from_utf8(vec![64u8; 50]).unwrap();
    let pt3 = BFVPlaintext::new(&s3, &bfv);
    let pt4 = BFVPlaintext::new(&s3, &bfv);

    let ct3 = bfv.encrypt(&pt3, &pk_a, &pk_b);
    let ct4 = bfv.encrypt(&pt4, &pk_a, &pk_b);

    let ct_sum2 = bfv.sum_ciphertexts(ct3, ct4);
    let result2 = bfv.backend_decrypt(&ct_sum2, &sk);

    for i in 0..50 {
        assert_eq!(result2.coeffs[i], 128, "Addition failed at index {}", i);
    }
}

#[test]
fn test_add_cipher_plain() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let pt1 = BFVPlaintext::new_from_coeffs(vec![60u64; n], &bfv);
    let pt2 = BFVPlaintext::new_from_coeffs(vec![30u64; n], &bfv);

    let ct1 = bfv.encrypt(&pt1, &pk_a, &pk_b);
    let result = bfv.backend_decrypt(
        &bfv.sum_ciphertext_and_plaintext(&ct1, &pt2),
        &sk
    );

    for i in 0..n.min(50) {
        assert_eq!(result.coeffs[i], 90, "C+P addition failed at index {}", i);
    }
}

#[test]
fn test_mul_cipher_plain() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let pt1 = BFVPlaintext::new_from_coeffs(vec![10u64; n], &bfv);

    let mut scalar = vec![0u64; n];
    scalar[0] = 3;
    let pt_scalar = BFVPlaintext::new_from_coeffs(scalar, &bfv);

    let ct1 = bfv.encrypt(&pt1, &pk_a, &pk_b);

    let result = bfv.backend_decrypt(
        &bfv.mul_ciphertext_and_plaintext(&ct1, &pt_scalar),
        &sk
    );

    for i in 0..n.min(50) {
        assert_eq!(result.coeffs[i], 30, "C*P multiplication failed at index {}", i);
    }
}

#[test]
fn test_add_then_mul() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);

    let pt_a = BFVPlaintext::new_from_coeffs(vec![20u64; bfv.params.n], &bfv);
    let pt_b = BFVPlaintext::new_from_coeffs(vec![10u64; bfv.params.n], &bfv);

    let mut scalar = vec![0u64; bfv.params.n];
    scalar[0] = 3;
    let pt_s = BFVPlaintext::new_from_coeffs(scalar, &bfv);

    let ct_sum = bfv.sum_ciphertexts(
        bfv.encrypt(&pt_a, &pk_a, &pk_b),
        bfv.encrypt(&pt_b, &pk_a, &pk_b),
    );

    let ct_res = bfv.mul_ciphertext_and_plaintext(&ct_sum, &pt_s);
    let result = bfv.backend_decrypt(&ct_res, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 90, "(A+B)*C failed at index {}", i);
    }
}

#[test]
fn test_mul_then_add() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);

    let pt5 = BFVPlaintext::new_from_coeffs(vec![5u64; bfv.params.n], &bfv);
    let pt7 = BFVPlaintext::new_from_coeffs(vec![7u64; bfv.params.n], &bfv);

    let mut s = vec![0u64; bfv.params.n];
    s[0] = 4;
    let pt_s = BFVPlaintext::new_from_coeffs(s, &bfv);

    let ct_mul = bfv.mul_ciphertext_and_plaintext(
        &bfv.encrypt(&pt5, &pk_a, &pk_b),
        &pt_s,
    );

    let ct_res = bfv.sum_ciphertexts(ct_mul, bfv.encrypt(&pt7, &pk_a, &pk_b));
    let result = bfv.backend_decrypt(&ct_res, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 27, "A*B+C failed at index {}", i);
    }
}

#[test]
fn test_chained_additions() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);

    let pt = BFVPlaintext::new_from_coeffs(vec![1u64; bfv.params.n], &bfv);

    let mut acc = bfv.encrypt(&pt, &pk_a, &pk_b);

    for _ in 1..10 {
        acc = bfv.sum_ciphertexts(
            acc,
            bfv.encrypt(&pt, &pk_a, &pk_b)
        );
    }

    let result = bfv.backend_decrypt(&acc, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 10, "Chained addition failed at index {}", i);
    }
}

#[test]
fn test_edge_zero() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);

    let pt_x = BFVPlaintext::new_from_coeffs(vec![128u64; bfv.params.n], &bfv);
    let pt_z = BFVPlaintext::new_from_coeffs(vec![0u64; bfv.params.n], &bfv);

    let ct = bfv.sum_ciphertexts(
        bfv.encrypt(&pt_x, &pk_a, &pk_b),
        bfv.encrypt(&pt_z, &pk_a, &pk_b),
    );

    let result = bfv.backend_decrypt(&ct, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 128, "Zero addition failed at index {}", i);
    }
}

#[test]
fn test_edge_max_value() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);

    let pt = BFVPlaintext::new_from_coeffs(vec![255u64; bfv.params.n], &bfv);
    let ct = bfv.encrypt(&pt, &pk_a, &pk_b);
    let result = bfv.backend_decrypt(&ct, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 255, "Max value decryption failed at index {}", i);
    }
}

#[test]
fn test_edge_add_one_to_max() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);

    let t = bfv.t;

    let pt_max = BFVPlaintext::new_from_coeffs(vec![t - 1; bfv.params.n], &bfv);
    let pt_one = BFVPlaintext::new_from_coeffs(vec![1u64; bfv.params.n], &bfv);

    let ct = bfv.sum_ciphertexts(
        bfv.encrypt(&pt_max, &pk_a, &pk_b),
        bfv.encrypt(&pt_one, &pk_a, &pk_b),
    );

    let result = bfv.backend_decrypt(&ct, &sk);

    for i in 0..50 {
        assert_eq!(result.coeffs[i], 0, "Overflow wrap failed at index {}", i);
    }
}

// "Roundtrip" tests

#[test]
fn test_roundtrip_full_coeffs() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let coeffs: Vec<u64> = (0..n).map(|i| (i % 200) as u64).collect();
    let plaintext = BFVPlaintext::new_from_coeffs(coeffs.clone(), &bfv);

    let ciphertext = bfv.encrypt(&plaintext, &pk_a, &pk_b);
    let recovered = bfv.backend_decrypt(&ciphertext, &sk);

    assert_eq!(recovered.coeffs, plaintext.plain.coeffs, "Full roundtrip failed");
}

#[test]
fn test_roundtrip_random_small_values() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let mut rng = std::collections::hash_map::DefaultHasher::new();
    let coeffs: Vec<u64> = (0..n)
        .map(|i| {
            use std::hash::{Hash, Hasher};
            i.hash(&mut rng);
            (rng.finish() % (bfv.t / 2)) as u64
        })
        .collect();

    let plaintext = BFVPlaintext::new_from_coeffs(coeffs.clone(), &bfv);
    let ciphertext = bfv.encrypt(&plaintext, &pk_a, &pk_b);
    let recovered = bfv.backend_decrypt(&ciphertext, &sk);

    assert_plaintext_eq(&recovered, &plaintext.plain, n);
}

#[test]
fn test_roundtrip_binary_plaintext() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    // Plaintext binaire : idéal pour les opérations multiples
    let coeffs: Vec<u64> = (0..n).map(|i| (i % 2) as u64).collect();
    let plaintext = BFVPlaintext::new_from_coeffs(coeffs.clone(), &bfv);

    let ciphertext = bfv.encrypt(&plaintext, &pk_a, &pk_b);
    let recovered = bfv.backend_decrypt(&ciphertext, &sk);

    assert_eq!(recovered.coeffs, plaintext.plain.coeffs, "Binary roundtrip failed");
}

//Noise "budget" tests :

#[test]
fn test_noise_budget_additions() {
    let bfv = BFV::for_medium(); 
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let pt = BFVPlaintext::new_from_coeffs(vec![1u64; n], &bfv);
    let mut ct = bfv.encrypt(&pt, &pk_a, &pk_b);

    let mut success_count = 0;
    for i in 1..=100 {
        ct = bfv.sum_ciphertexts(ct.clone(), ct.clone());
        let result = bfv.backend_decrypt(&ct, &sk);
        
        let expected = (1u64 << i.min(7)) % bfv.t; 
        
        if result.coeffs[0] == expected {
            success_count = i;
        } else {
            println!("Decryption failed after {} additions: got {}, expected {}", 
                     i, result.coeffs[0], expected);
            break;
        }
    }

    println!("Successful additions before noise failure: {}", success_count);
    assert!(success_count >= 5, "Noise budget too small: failed after {} additions", success_count);
}

#[test]
fn test_noise_budget_multiplications() {
    let bfv = BFV::for_large(); 
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let mut m = vec![0u64; n];
    m[0] = 2;  
    let pt = BFVPlaintext::new_from_coeffs(m, &bfv);

    let mut scalar = vec![0u64; n];
    scalar[0] = 2;  
    let pt_scalar = BFVPlaintext::new_from_coeffs(scalar, &bfv);
    
    let mut ct = bfv.encrypt(&pt, &pk_a, &pk_b);

    let mut success_count = 0;
    for i in 1..=15 {
        ct = bfv.mul_ciphertext_and_plaintext(&ct, &pt_scalar);
        let result = bfv.backend_decrypt(&ct, &sk);
        
        let expected = (2u64.pow(i as u32 + 1)) % bfv.t;
        
        if result.coeffs[0] == expected {
            success_count = i;
        } else {
            println!("Decryption failed after {} multiplications: got {}, expected {}", 
                     i, result.coeffs[0], expected);
            break;
        }
    }

    println!("Successful multiplications before noise failure: {}", success_count);
    assert!(success_count >= 8, "Failed after {} multiplications", success_count);
}

#[test]
fn test_noise_budget_mixed_operations() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let pt_base = BFVPlaintext::new_from_coeffs(vec![3u64; n], &bfv);
    let pt_add = BFVPlaintext::new_from_coeffs(vec![1u64; n], &bfv);
    let pt_mul = BFVPlaintext::new_from_coeffs(vec![2u64; n], &bfv);

    let mut ct = bfv.encrypt(&pt_base, &pk_a, &pk_b);
    let mut operations = 0;

    for i in 0..15 {
        if i % 2 == 0 {
            ct = bfv.sum_ciphertexts(ct.clone(), bfv.encrypt(&pt_add, &pk_a, &pk_b));
        } else {
            ct = bfv.mul_ciphertext_and_plaintext(&ct, &pt_mul);
        }
        operations += 1;

        let result = bfv.backend_decrypt(&ct, &sk);
        assert!(result.coeffs[0] < bfv.t, "Decryption overflow after {} ops", operations);
    }

    println!("Completed {} mixed operations without decryption failure", operations);
}

//Noise estimation tests

#[test]
fn test_noise_estimation_growth() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let pt = BFVPlaintext::new_from_coeffs(vec![1u64; n], &bfv);
    let mut ct = bfv.encrypt(&pt, &pk_a, &pk_b);

    let mut noises = Vec::new();
    for i in 0..15 {
        let noise = bfv.estimate_noise(&sk, &bfv.params, &ct);
        noises.push(noise);
        println!("After {} additions, estimated noise: {}", i, noise);
        ct = bfv.sum_ciphertexts(ct.clone(), ct.clone());
    }

    assert!(noises[10] >= noises[0], "Noise should grow with operations");
}

#[test]
fn test_noise_vs_delta_threshold() {
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;

    let delta = bfv.params.q / bfv.t;
    let threshold = delta / 2;  

    let pt = BFVPlaintext::new_from_coeffs(vec![1u64; n], &bfv);
    let mut ct = bfv.encrypt(&pt, &pk_a, &pk_b);

    for i in 0..50 {
        let noise = bfv.estimate_noise(&sk, &bfv.params, &ct);
        
        if noise < threshold {
            // Décryption devrait fonctionner
            let result = bfv.backend_decrypt(&ct, &sk);
            assert!(result.coeffs[0] < bfv.t, "Decryption failed while noise < threshold");
        } else {
            println!("Noise exceeded threshold ({}) after {} additions: {}", threshold, i, noise);
            break;
        }
        
        ct = bfv.sum_ciphertexts(ct.clone(), ct.clone());
    }
}


//Parameters validation tests

#[test]
#[should_panic(expected = "NTT requires n to be a power of 2, got 1028 ! For explanations, please read '/docs/simple-ring.pdf")]
fn test_invalid_n_not_power_of_two() {
    let _ = simple_ring::RingParams::new(1028, 786_433, 1);
}

#[test]
fn test_valid_configurations() {
    BFV::for_test();   
    BFV::for_medium(); 
    BFV::for_large();  
}

#[test]
fn test_parameter_consistency() {
    let bfv = BFV::for_test();
    
    assert!(bfv.params.n.is_power_of_two());
    assert!(bfv.t >= 2 && bfv.t < bfv.params.q);
    assert!(bfv.eta >= 1 && bfv.eta <= 16);
    
    assert_eq!(bfv.ntt_precalculated.twiddles.len(), bfv.params.n);
    assert_eq!(bfv.ntt_precalculated.twiddles_inv.len(), bfv.params.n);
}

// Performances tests (not really precise as real benchmarks)

#[test]
#[ignore] //Ignore by default, it can be really slow 
fn test_performance_encrypt_decrypt() {
    let bfv = BFV::for_large(); 
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    
    let message = BFVPlaintext::new_from_coeffs(vec![42u64; bfv.params.n], &bfv);
    
    let start = Instant::now();
    let ct = bfv.encrypt(&message, &pk_a, &pk_b);
    let enc_time = start.elapsed();
    
    let start = Instant::now();
    let _ = bfv.backend_decrypt(&ct, &sk);
    let dec_time = start.elapsed();
    
    println!("Encrypt time (n=4096): {:?}", enc_time);
    println!("Decrypt time (n=4096): {:?}", dec_time);
    
    // Seuils approximatifs (à ajuster selon ta machine)
    assert!(enc_time.as_millis() < 100, "Encryption too slow: {:?}", enc_time);
    assert!(dec_time.as_millis() < 100, "Decryption too slow: {:?}", dec_time);
}

#[test]
#[ignore]
fn test_performance_homomorphic_add() {
    let bfv = BFV::for_large();
    let (pk_a, pk_b, _sk) = generate_keys(&bfv);
    
    let pt = BFVPlaintext::new_from_coeffs(vec![1u64; bfv.params.n], &bfv);
    let ct1 = bfv.encrypt(&pt, &pk_a, &pk_b);
    let ct2 = bfv.encrypt(&pt, &pk_a, &pk_b);
    
    let start = Instant::now();
    let _ = bfv.sum_ciphertexts(ct1, ct2);
    let add_time = start.elapsed();
    
    println!("Homomorphic add time (n=4096): {:?}", add_time);
    assert!(add_time.as_millis() < 50, "Addition too slow: {:?}", add_time);
}


//Other tests :

#[test]
fn test_empty_plaintext() { //Just encrypt empty plaintexts
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    
    let pt = BFVPlaintext::new_from_coeffs(vec![0u64; bfv.params.n], &bfv);
    let ct = bfv.encrypt(&pt, &pk_a, &pk_b);
    let result = bfv.backend_decrypt(&ct, &sk);
    
    for i in 0..50 {
        assert_eq!(result.coeffs[i], 0, "Empty plaintext decryption failed");
    }
}

#[test]
fn test_single_coefficient_nonzero() { 
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    let n = bfv.params.n;
    
    let mut coeffs = vec![0u64; n];
    coeffs[n/2] = 123; 
    
    let pt = BFVPlaintext::new_from_coeffs(coeffs.clone(), &bfv);
    let ct = bfv.encrypt(&pt, &pk_a, &pk_b);
    let result = bfv.backend_decrypt(&ct, &sk);
    
    assert_eq!(result.coeffs[n/2], 123, "Single coefficient decryption failed");
}

#[test]
fn test_deterministic_encryption_with_same_randomness() {
    // BFV is probabilistic : even with the same keys, because of the noise, a plaintext shouldn't give two same ciphertexts
    
    let bfv = BFV::for_test();
    let (pk_a, pk_b, sk) = generate_keys(&bfv);
    
    let pt = BFVPlaintext::new_from_coeffs(vec![42u64; bfv.params.n], &bfv);
    
    let ct1 = bfv.encrypt(&pt, &pk_a, &pk_b);
    let ct2 = bfv.encrypt(&pt, &pk_a, &pk_b);
    
    assert_ne!(ct1.c0.coeffs, ct2.c0.coeffs, "Encryption should be probabilistic");
    
    let res1 = bfv.backend_decrypt(&ct1, &sk);
    let res2 = bfv.backend_decrypt(&ct2, &sk);
    assert_eq!(res1.coeffs, res2.coeffs, "Decryption should be deterministic");
}

