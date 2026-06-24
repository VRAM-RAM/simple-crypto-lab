
use crate::{Saber, error::SaberError};
use simple_ring::{
    ToPoly
};
use rand::rngs::OsRng;
use rand::RngCore;
use crate::{SaberCiphertext, SaberKeypair, SaberPublicKey};
use subtle::{ConstantTimeEq};

#[derive(Debug, Clone)]
pub struct SaberEncapsulated {
    pub ciphertext: SaberCiphertext,
    pub mac: Box<[u8]>,
}

impl Saber {
    pub fn encapsulate(&self, public_key: &SaberPublicKey) -> SaberEncapsulated {
        let mut key = vec![0u8; 32];
        OsRng.fill_bytes(&mut key);
        let poly = &key.as_slice().to_poly();
        let ciphertext = self.encrypt(public_key,&poly);
        let mut mac_plain = public_key.0.as_bytes().to_vec();
        mac_plain.append(&mut key);
        let mac = blake3::hash(&mac_plain).as_bytes().to_vec().into_boxed_slice();
        SaberEncapsulated { ciphertext, mac }
    }

    pub fn decapsulate(&self, keypair: &SaberKeypair, encapsulated: SaberEncapsulated) -> Result<Vec<u8>, SaberError> {
        let poly = self.decrypt(keypair, &encapsulated.ciphertext);
        let mut key = poly.as_bytes().to_vec();
        let mut mac_plain = keypair.public_key.0.as_bytes().to_vec();
        mac_plain.append(&mut key);
        let mac = blake3::hash(&mac_plain).as_bytes().to_vec().into_boxed_slice();
        let result = mac.ct_eq(&encapsulated.mac);
        if result.into() {
            Ok(key)
        } else {
            Err(SaberError::InvalidMac)
        }
    }
}