
use crate::{Saber};
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


#[derive(Debug)]
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

    pub fn encode(&self, params: &RingParams, m: &Polynomial) -> Polynomial {
        let p = self.p;       // ex: 64 
        let t = self.t;       // ex: 2
        let factor = p / t;   // ex: 32
        
        m.scale(params, factor)
    }

    pub fn decode(&self, params: &RingParams, encoded: &Polynomial) -> Polynomial {
        let p = self.p;       // ex: 64
        let quarter_p = p / 4;       // 16
        let three_quarter_p = 3 * p / 4; // 48
        let mut coeffs = vec![0u64; params.n];
    
        for i in 0..params.n {
            let raw = encoded.coeffs[i] % p;
            if raw >= quarter_p && raw < three_quarter_p {
                coeffs[i] = 1;
            } else {
                coeffs[i] = 0;
            }
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

        let a = generate_then_shake(params, simple_ring::SeedType::Given(seed)).0.to_poly(q);
        
        let r = generate_cbd_sample(n, self.eta).to_poly(q);
        
        let mut r_compressed = vec![0u64; n];
        for i in 0..n {
            let val = r.coeffs[i] as i64;
            r_compressed[i] = ((val % self.p as i64) + self.p as i64).rem_euclid(self.p as i64) as u64;
        }
        let r_compressed = Polynomial::new(r_compressed);

        let u = a.mul_ntt(params, ntt_tables, &r_compressed);
        let u = self.round(params, u);

        let v = b.mul_ntt(params, ntt_tables, &r_compressed); 
        let v = self.round(params, v);

        let encoded_message = self.encode(params, message);
        
        let v_message = v.sum(params, &encoded_message);

        SaberCiphertext { v: v_message, u }
    }

    //The saber decryption
    pub fn decrypt(&self, keypair: &SaberKeypair, encapsulated: &SaberCiphertext) -> Polynomial {
        let params = &self.params;
        let ntt_tables = &self.ntt_precalculated;

        let s = &keypair.secret_key;
        let u = &encapsulated.u;
        let v = &encapsulated.v;

        let su = s.mul_ntt(params, ntt_tables, u);
        let su = self.round(params, su);  

        let m_poly = v.sub(params, &su);

        self.decode(params, &m_poly)
    }

    pub fn decapsulate(&self, keypair: SaberKeypair, ciphertext: SaberCiphertext) -> Key {
        let decrypted = self.decrypt(&keypair, &ciphertext);
        let mut bytes = vec![0u8; self.params.n / 8];
        for (i, &coeff) in decrypted.coeffs.iter().enumerate() {
            if coeff & 1 == 1 { 
                bytes[i / 8] |= 1 << (i % 8);
            }
        }
        derive_key("SaberKeyDerivation", &bytes)
    }
}










