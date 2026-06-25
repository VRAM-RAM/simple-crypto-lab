mod config;
mod scheme;
mod encapsulation;
mod error;
mod types;
mod exportable_params;

pub use config::{Saber};
pub use scheme::{Key, SaberCiphertext};
pub use crate::types::{SaberKeypair, SaberPublicKey, SaberSecretKey};
pub use crate::encapsulation::SaberEncapsulated;
pub use crate::encapsulation::ToSaberEncapsulated;
pub use crate::encapsulation::SaberDecapsulate;


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
    
    let msg = Polynomial::random_binary(saber.params.n);
    
    let ct = saber.encrypt(&kp.public_key, &msg);
    let decrypted = saber.decrypt(&kp, &ct);
    
    assert_eq!(msg.coeffs, decrypted.coeffs, "decrypt(encrypt(m)) != m");
}

#[cfg(test)]
#[test]
fn test_encapdecap() {
    let saber = Saber::fire();
    let kp = saber.keygen();
    let (key, serialized_enc) = match saber.encapsulate(&kp.public_key) {
        Ok((key, serialized)) => (key, serialized),
        Err(e) => panic!("Error while encapsulating : {:?}", e),
    };
    let recovered = match serialized_enc.decapsulate(&kp) {
        Ok(key) => key,
        Err(e) => { panic!("Error while decapsulating : {:?}", e) }
    };
    assert_eq!(key, recovered, "Error : the encapsulated key and the decapsulated key doesn't match !")
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



