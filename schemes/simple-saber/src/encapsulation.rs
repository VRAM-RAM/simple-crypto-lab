use crate::{Saber, error::SaberError, exportable_params::SaberExportated};
use simple_ring::{
    ToPoly
};
use rand::rngs::OsRng;
use rand::RngCore;
use crate::{SaberCiphertext, SaberKeypair, SaberPublicKey};
use subtle::{ConstantTimeEq};

#[derive(Clone)]
pub struct SaberEncapsulated {
    pub ciphertext: SaberCiphertext,
    pub mac: Box<[u8]>,
    pub exportated: SaberExportated,
}

impl Saber {
    pub fn encapsulate(&self, public_key: &SaberPublicKey) -> (Vec<u8>, SaberEncapsulated) {
        let mut key = vec![0u8; 32];
        OsRng.fill_bytes(&mut key);
        let poly = &key.as_slice().to_poly();
        let ciphertext = self.encrypt(public_key,&poly);
        let mut mac_plain = public_key.0.as_bytes().to_vec();
        mac_plain.extend_from_slice(&key);
        let mac = blake3::hash(&mac_plain).as_bytes().to_vec().into_boxed_slice();
        let exportated = self.to_export();
        (key ,SaberEncapsulated { ciphertext, mac, exportated })
    }


}

impl SaberEncapsulated {
    pub fn decapsulate(&self, keypair: &SaberKeypair) -> Result<Vec<u8>, SaberError> {
        let saber = self.exportated.to_saber();
        let poly = saber.decrypt(keypair, &self.ciphertext);
        let key = poly.as_bytes().to_vec();
        let mut mac_plain = keypair.public_key.0.as_bytes().to_vec();
        mac_plain.extend_from_slice(&key);
        let mac = blake3::hash(&mac_plain).as_bytes().to_vec().into_boxed_slice();
        let result = mac.ct_eq(&self.mac);
        if result.into() {
            Ok(key)
        } else {
            Err(SaberError::InvalidMac)
        }
    }
}