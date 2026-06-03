
use crate::{Saber};
use rand::{RngCore, rngs::OsRng};
use simple_ring::{Polynomial, RingParams, generate_cbd_sample, generate_small_sample, generate_then_shake};
use blake3::derive_key;

pub struct SaberKeypair {
    pub public_key: SaberPublicKey,
    pub secret_key: SaberSecretKey,
}

pub type SaberPublicKey = (Polynomial, [u8; 32]);
pub type SaberSecretKey = Polynomial;

impl SaberKeypair {
    pub fn new(b: Polynomial, seed: [u8; 32], s: Polynomial) -> Self {
        Self { public_key: (b, seed), secret_key: s }
    }
}



pub struct SaberCiphertext {
    pub v: Polynomial,
    pub u: Polynomial,
}

pub type  Key = [u8; 32];

impl Saber {
    //Needed small methods 
    fn round(&self, params: &RingParams, a: Polynomial) -> Polynomial {
        let n = params.n;
        let q  = params.q;
        let p = self.p;

        let mut result = vec![0u64; n];
        for i in 0..n {
            result[i] = (( a.coeffs[i] * p ) + q/2 ) / q;
        }
        Polynomial::new(result)
    }

    fn encode(&self, params: &RingParams, m: &Polynomial) -> Polynomial {
        let p = self.p;
        let t = self.t;
        let factor = p / t;
        m.scale(params, factor)
    }

    fn decode(&self, params: &RingParams, encoded: &Polynomial) -> Polynomial {
        let p = self.p as i128;
        let t = self.t as i128;
        let mut coeffs = vec![0u64; params.n];
        for i in 0..params.n {
            let raw = encoded.coeffs[i] as i128;
            
            let decoded = ((raw * t + p / 2).div_euclid(p) % t + t) % t ;

            coeffs[i] = decoded as u64;
        }
        Polynomial::new(coeffs)
    }

    //The key generation /docs/simple-saber
    pub fn keygen(&self) -> SaberKeypair {
        let params = &self.params;
        let q = params.q;
        let ntt_tables = &self.ntt_precalculated;

        let (a, seed) = generate_then_shake(params, simple_ring::SeedType::NotGiven);
        let a = a.to_poly(q);

        let s = generate_small_sample(params).to_poly(q);
       
        let b = a.mul_ntt(params, ntt_tables, &s);
        let b = b.divide_by_constant(self.p as u128);
        let b = self.round(params, b);

        SaberKeypair::new(b, seed, s)
    }

    //The saber encryption
    pub fn encrypt(&self, public_key: &SaberPublicKey, message: &Polynomial) -> SaberCiphertext {
        let seed = public_key.1;
        let b = &public_key.0;

        let params = &self.params;
        let q = params.q;
        let n = params.n;
        let ntt_tables = &self.ntt_precalculated;

        let a = generate_then_shake(params, simple_ring::SeedType::Given(seed)).0.to_poly(q); //Yes, it's barely unreadable, but it works
        
        let r = generate_cbd_sample(n, self.eta).to_poly(q);

        let u = a.mul_ntt(params, ntt_tables,&r);
        let u = self.round(params, u);

        let v = b.mul_ntt(params, ntt_tables,&r); 
        let v = self.round(params, v);

        let encoded_message = self.encode(params, message);
        let v_message: Polynomial = v.sum(params, &encoded_message);

        SaberCiphertext { v: v_message, u }
    }

    //The saber decryption
    pub fn decrypt(&self, keypair: &SaberKeypair, encapsulated: &SaberCiphertext) -> Polynomial {
        let params = &self.params;
        let ntt_tables = &self.ntt_precalculated;

        let s = &keypair.secret_key;
        let u = &encapsulated.u;
        let v = &encapsulated.v;

        let su = s.mul_ntt(params, ntt_tables, &u);
        
        let encoded_message = v.sub(params, &su);

        self.decode(params, &encoded_message)
    }

    pub fn encapsulate(&self, public_key: SaberPublicKey) -> (SaberCiphertext, Key) {
        let n = self.params.n;

        let mut buffer = vec![0u8; n];
        let mut rng = OsRng;
        rng.fill_bytes(&mut buffer);

        let coeffs = buffer.iter().map(|v| *v as u64).collect();
        let keypoly = Polynomial::new(coeffs);

        assert_eq!(keypoly.coeffs.len(), n, "The size of the key and the size defined in the parameters don't match.");

        let encrypted = self.encrypt(&public_key, &keypoly); 

        let key = derive_key("SaberKeyDerivation", &buffer);

        (encrypted, key)
    }

    pub fn decapsulate(&self, keypair: SaberKeypair, ciphertext: SaberCiphertext) -> Key {
        let decrypted = self.decrypt(&keypair, &ciphertext);
        let bytes: Vec<u8> = decrypted.coeffs.iter().map(|v| *v as u8).collect();

        derive_key("SaberKeyDerivation", &bytes)
    }
}










