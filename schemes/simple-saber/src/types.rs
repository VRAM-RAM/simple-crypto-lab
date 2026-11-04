use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::Poly64;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The public key, that stores the key B which is a polynomial, and the Seed for generating the key A.
pub struct SaberPublicKey {
    pub b: Poly64,
    pub a_seed: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The scecret key
pub struct SaberSecretKey(Poly64);

impl Deref for SaberSecretKey {
    type Target = Poly64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The Keypair, that stores the two keys (more simple to manipulate)
pub struct SaberKeypair {
    pub public_key: SaberPublicKey,
    pub secret_key: SaberSecretKey,
}

impl SaberKeypair {
    pub fn new(b: Poly64, seed: [u8; 32], s: Poly64) -> Self {
        Self {
            public_key: SaberPublicKey { b, a_seed: seed },
            secret_key: SaberSecretKey(s),
        }
    }
}
