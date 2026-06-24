use simple_ring::Polynomial;

pub type SaberPublicKey = (Polynomial, [u8; 32]); //The public key stores the key B which is a polynomial, and the Seed for generating the key A.
pub type SaberSecretKey = Polynomial; //The secret key S

#[derive(Debug, Clone)]
pub struct SaberKeypair { //The Keypair, that stores the two keys (more simple to manipulate)
    pub public_key: SaberPublicKey,
    pub secret_key: SaberSecretKey,
}

impl SaberKeypair {
    pub fn new(b: Polynomial, seed: [u8; 32], s: Polynomial) -> Self {
        Self { public_key: (b, seed), secret_key: s }
    }
}