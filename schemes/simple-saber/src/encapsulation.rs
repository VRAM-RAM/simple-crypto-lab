use crate::{Saber, error::SaberError, exportable_params::SaberExportated};
use serde::{Deserialize, Serialize};
use simple_ring::{
    Polynomial
};
use core::ops::Deref;
use postcard::{from_bytes, to_allocvec};
use crate::{SaberCiphertext, SaberKeypair, SaberPublicKey};
use subtle::{ConstantTimeEq};

/*
Saber-like encapsulation, without SO-transform nor real security
*/
#[derive(Clone, Serialize, Deserialize)]
pub struct SaberEncapsulated {
    pub ciphertext: SaberCiphertext, //The ciphertext
    pub mac: Box<[u8]>, //The mac : hash(pk, secret)
    pub exportated: SaberExportated, //The exportated parameters : we don't export the ntt tables (too large)
}

impl SaberEncapsulated {
    pub fn new(ciphertext: SaberCiphertext, mac: Box<[u8]>, exportated: SaberExportated) -> Self {
        Self { ciphertext, mac, exportated }
    }
}

impl Saber {

    /*
    The encapsulation function. We first generate the key, which is already encoded as a Polynomial. Then, we encrypt it, 
    build the MAC (pk || secret), that we hash. We finally return the key, in bytes (Vec<u8>) for the user, and the encapsulated structure for the peer.
    We also serialize the data so that they can be sent. 
     */ 

    pub fn encapsulate(&self, public_key: &SaberPublicKey) -> Result<(Vec<u8>, Vec<u8>), SaberError> { 
        let key = Polynomial::random_binary(self.params.n);
        let ciphertext = self.encrypt(public_key,&key);
        let mut mac_plain = public_key.b.as_bytes().to_vec();
        mac_plain.extend_from_slice(key.as_bytes());
        let mac = blake3::hash(&mac_plain).as_bytes().to_vec().into_boxed_slice();
        let exportated = self.to_export();
        let saber_enc = SaberEncapsulated::new(ciphertext, mac, exportated);
        let bytes_enc = match to_allocvec(&saber_enc) {
            Ok(v) => v,
            Err(e) => return Err(SaberError::SerializationError(e))
        };
        let shared_key = blake3::hash(key.as_bytes()).as_bytes().to_vec();
        Ok((shared_key,  bytes_enc))
    }
}

/*
To deserialize a Vec<u8> to a SaberEncapsulated
*/

#[allow(dead_code)]
pub trait ToSaberEncapsulated {
    fn to_saber_encap(&self) -> Result<SaberEncapsulated, SaberError>;
}

impl ToSaberEncapsulated for Vec<u8> {
    fn to_saber_encap(&self) -> Result<SaberEncapsulated, SaberError> {
        match from_bytes::<SaberEncapsulated>(self.deref()) {
            Ok(v) => Ok(v),
            Err(e) => return Err(SaberError::SerializationError(e))
        }
    }
}

/*
To decapsulate, either from a Vec<u8> (a SaberEncapsulated serialized) or a SaberEncapsulated
*/

#[allow(dead_code)]
pub trait SaberDecapsulate {
    fn decapsulate(&self, keypair: &SaberKeypair) -> Result<Vec<u8>, SaberError>;
}

impl SaberDecapsulate for SaberEncapsulated {
     /*
    The decapsulation function. We first rebuild the saber struct, decrypt the ciphertext, decode the key, and recompute the MAC. 
    Then, with a constant-time equation, we determine if the MAC is valid or not.
     */

    fn decapsulate(&self, keypair: &SaberKeypair) -> Result<Vec<u8>, SaberError> {
        let saber = self.exportated.to_saber();
        let poly = saber.decrypt(keypair, &self.ciphertext);
        let key = poly.as_bytes().to_vec();
        let mut mac_plain = keypair.public_key.b.as_bytes().to_vec();
        mac_plain.extend_from_slice(&key);
        let mac = blake3::hash(&mac_plain).as_bytes().to_vec().into_boxed_slice();
        let result = mac.ct_eq(&self.mac);
        if result.into() {
            let shared_key = blake3::hash(&key).as_bytes().to_vec();
            Ok(shared_key)
        } else {
            Err(SaberError::InvalidMac)
        }
    }
}

impl SaberDecapsulate for Vec<u8> {
    fn decapsulate(&self, keypair: &SaberKeypair) -> Result<Vec<u8>, SaberError> {
        let encap = self.to_saber_encap()?; //The same, we just deserialize the SaberEncapsulated
        let saber = encap.exportated.to_saber();
        let poly = saber.decrypt(keypair, &encap.ciphertext);
        let key = poly.as_bytes().to_vec();
        let mut mac_plain = keypair.public_key.b.as_bytes().to_vec();
        mac_plain.extend_from_slice(&key);
        let mac = blake3::hash(&mac_plain).as_bytes().to_vec().into_boxed_slice();
        let result = mac.ct_eq(&encap.mac);
        if result.into() {
            let shared_key = blake3::hash(&key).as_bytes().to_vec();
            Ok(shared_key)
        } else {
            Err(SaberError::InvalidMac)
        }
    }
}