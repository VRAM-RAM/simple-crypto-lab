use serde::{Deserialize, Serialize};
use simple_ring::Polynomial;
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaberPublicKey { 
    pub b: Polynomial, 
    pub a_seed: [u8; 32] 
} //The public key stores the key B which is a polynomial, and the Seed for generating the key A.


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaberSecretKey(Polynomial); //The secret key S


impl Deref for SaberSecretKey {
    type Target = Polynomial;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaberKeypair { //The Keypair, that stores the two keys (more simple to manipulate)
    pub public_key: SaberPublicKey,
    pub secret_key: SaberSecretKey,
}

impl SaberKeypair {
    pub fn new(b: Polynomial, seed: [u8; 32], s: Polynomial) -> Self {
        Self { public_key: SaberPublicKey { b, a_seed: seed }, secret_key: SaberSecretKey(s) }
    }
}