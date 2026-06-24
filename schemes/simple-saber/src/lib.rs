mod config;
mod scheme;
mod encapsulation;
mod error;
pub use config::{Saber};
pub use scheme::{Key, SaberCiphertext, SaberKeypair, SaberPublicKey};

/*
This crate provides a simplified implementation of SABER KEM scheme. It was rebuilt to simplify implementation, and to use `simple-ring`. 
For example :

- SABER doesn't normally use the NTT nor a NTT-friendly ciphertext modulus q. In this implementation, I decided to use the NTT & 
in consequence a compatible ciphertext modulus. 

- SABER uses other parameters, like T (a factor used in compression and encoding)
*/
#[cfg(test)]
#[test]
fn test_encrypt_decrypt_roundtrip() {
    use simple_ring::Polynomial;
    let saber = Saber::light();
    let kp = saber.keygen();
    
    let msg = Polynomial::random(saber.params.n);
    
    let ct = saber.encrypt(&kp.public_key, &msg);
    let decrypted = saber.decrypt(&kp, &ct);
    
    assert_eq!(msg.coeffs, decrypted.coeffs, "decrypt(encrypt(m)) != m");
}

#[test]
fn test_encode_decode_roundtrip() {
    use rand::{RngCore, rngs::OsRng};

    use simple_ring::Polynomial;
    let saber = Saber::light(); 
    let params = &saber.params;
    
    let mut rng = OsRng;
    let mut coeffs = vec![0u64; params.n];
    for c in coeffs.iter_mut() {
        *c = rng.next_u32() as u64 & 1;
    }
    let msg = Polynomial::new(coeffs);
    
    let encoded = saber.encode(params, &msg);
    let decoded = saber.decode(params, &encoded);
    
    assert_eq!(msg.coeffs, decoded.coeffs, "encode/decode are not inverse");
}



