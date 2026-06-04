mod config;
mod scheme;

pub use config::{Saber};
pub use scheme::{Key, SaberCiphertext, SaberKeypair, SaberPublicKey};

#[cfg(test)]
#[test]
fn test_encrypt_decrypt_roundtrip() {
    use rand::{RngCore, rngs::OsRng};

    use simple_ring::Polynomial;
    let saber = Saber::recommended();
    let kp = saber.keygen();
    
    // Message binaire
    let mut rng = OsRng;
    let mut coeffs = vec![0u64; saber.params.n];
    for c in coeffs.iter_mut() {
        *c = rng.next_u32() as u64 & 1;
    }
    let msg = Polynomial::new(coeffs);
    
    let ct = saber.encrypt(&kp.public_key, &msg);
    let decrypted = saber.decrypt(&kp, &ct);
    
    assert_eq!(msg.coeffs, decrypted.coeffs, "decrypt(encrypt(m)) != m");
}

#[test]
fn test_encode_decode_roundtrip() {
    use rand::{RngCore, rngs::OsRng};

    use simple_ring::Polynomial;
    let saber = Saber::recommended(); 
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