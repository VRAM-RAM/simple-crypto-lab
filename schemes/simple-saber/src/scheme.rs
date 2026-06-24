use crate::Saber;
use simple_ring::{
    Polynomial, RingParams, generate_cbd_sample,
    generate_small_sample, generate_then_shake,
};
use crate::types::{SaberKeypair, SaberPublicKey};

/*
This file provides the core for the simple implementation of SABER scheme in Rust, using primitives given
in simple-ring. SABER is based on LWR (Learning With Roundness) problem. For more informations / explanations,
please read '/docs/pdf/simple-saber.pdf. (upcoming)
*/


#[derive(Debug, Clone)]
pub struct SaberCiphertext {
    pub v: Polynomial, //The ciphertext V
    pub u: Polynomial, //The other public key, noted as U or B' 
}

pub type Key = [u8; 32]; //An alias 

impl Saber {

    /// Intern but exposed functions for encryption/decryption (so that anyone can, from an external crate, use these methods)

    pub fn round(&self, params: &RingParams, a: Polynomial) -> Polynomial { //The Round function
        /*
        Let a be a coefficient as a ∈ Z_q.
        The round is â = round[ (a * p/q) ] mod p, so that â ∈ Z_p and not Z_q. Here, we don't manipulate floating numbers, so we compute the round as :
        
        - First the scaling with p : 

        â = a * p

        - Then, we add q/2 to simulate a mathematical rounding, and not doing a floor (because by default the compilator does a floor when rounding) :

        â = ap + q/2 

        - We divide by q :

        â =  ( ap + q/2 ) / q

        - We finally reduce by p :

        â = [ ( ap + q/2 ) / q ] mod p

        For example, with a = 1 , p = 5 and q = 100 :

        â = [ (12 * 5 + 50 ) / 100 ] mod 5
        â = [ 110 / 100 ] mod 5
        â = 1.10
        â = 1 (floor)

        If we had not done the + q /2 :

        â = [ 60/100 ] mod 5
        â = 0.60
        â = 0 (floor)
         */
        let n = params.n;
        let q = params.q;
        let p = self.p;

        let mut result = vec![0u64; n];
        for i in 0..n {
            result[i] = (((a.coeffs[i] * p) + q / 2) / q) % p;
        }
        Polynomial::new(result)
    }

    pub fn encode(&self, params: &RingParams, m: &Polynomial) -> Polynomial { //The encoding function
        /*
        The encoding, in Saber, is just the preojection of the message m, as m ∈ {0, 1} (because it's compound of binary coefficients), in Z_p
        To do that, we consider two cases : 
        
        - The coefficient is equal to 0 => the coefficient in Z_p is also 0

        - The coefficient is equal to 1 => the coefficient in Z_p becames equal to p/2 (centered in Z_p)

        To do that, we use the scaling, a polynomial operation, and we compute it with a factor p/2.
        (cause it's the same as matching coefficients, but mathematically : 0 * p/2 = 0 and 1 * p/2 = p/2)
         */
        
        let p = self.p;
        let factor = p / 2;
        let m = m.scale(params, factor); 
        m.reduce(p as u128)
    }

    pub fn decode(&self, params: &RingParams, encoded: &Polynomial) -> Polynomial {
        /*
        For decoding, we consider now that we have coefficients in Z_p, so c ∈ {0, 1, 2, 3, ..., p - 1}. 
        We can't do a simple matching as in the encoding function. 
        We consider now four bounds :
        {0, p/4, 3p/4, p} (which is the same as "cutting" Z_p into 4 equivalent pieces)

        Now, considering the coefficients were centered before, in the encoding, we two cases :

        - If the coefficients is in [p/4; 3p/4[ => it becames a 1

        - Else, it becames a 0
         */
        let p = self.p;
        let quarter_p = p / 4;
        let three_quarter_p = 3 * p / 4;
        let mut coeffs = vec![0u64; params.n];
        for i in 0..params.n {
            let raw = encoded.coeffs[i] % p; //The encoded is in Z_q, we first pass it in Z_p
            coeffs[i] = if raw >= quarter_p && raw < three_quarter_p { 1 } else { 0 };
        }
        Polynomial::new(coeffs)
    }

    pub fn compress(&self, poly: &Polynomial) -> Polynomial {
        let mut result = vec![0u64; poly.coeffs.len()];
        for (i, &c) in poly.coeffs.iter().enumerate() {
            let val = c as i64;
            let reduced = val % self.p as i64;
            result[i] = if reduced < 0 { (reduced + self.p as i64) as u64 } else { reduced as u64 };
        }
        Polynomial::new(result)
    }

    pub fn lift(&self, poly: &Polynomial) -> Polynomial {
        let result: Vec<u64> = poly.coeffs.iter().map(|&c| (c * self.params.q as u64) / self.p).collect(); //It goes back in Z_q
        Polynomial::new(result)
    }

    pub fn keygen(&self) -> SaberKeypair {
        let params = &self.params;
        let q = params.q;
        let ntt_tables = &self.ntt_precalculated;

        let (a, seed) = generate_then_shake(params, simple_ring::SeedType::NotGiven);
        let a = a.to_poly(q);

        let s = generate_small_sample(params).to_poly(q);

        let b_raw = a.mul_ntt(params, ntt_tables, &s);
        let b = self.round(params, b_raw);

        SaberKeypair::new(b, seed, s)
    }

    pub fn encrypt(&self, public_key: &SaberPublicKey, message: &Polynomial) -> SaberCiphertext {
        let seed = public_key.1;
        let b = &public_key.0; // In Z_p

        let params = &self.params;
        let q = params.q;
        let n = params.n;
        let ntt_tables = &self.ntt_precalculated;

        let a = generate_then_shake(params, simple_ring::SeedType::Given(seed)).0.to_poly(q); //In Z_q

        let r = generate_cbd_sample(n, self.eta).to_poly(q); //In Z_q

        let u_raw = a.mul_ntt(params, ntt_tables, &r); //In Z_q
        let u = self.round(params, u_raw); //Then returned in Z_p

        let b_in_q = self.lift(b);

        let v_raw = b_in_q.mul_ntt(params, ntt_tables, &r); //In Z_q
        let v = self.round(params, v_raw); //Then returned in Z_p

        let encoded_message = self.encode(params, message);
        let v_message = v.sum(params, &encoded_message);
        let v_message = v_message.reduce(self.p as u128);

        SaberCiphertext { v: v_message, u }
    }

    pub fn decrypt(&self, keypair: &SaberKeypair, encapsulated: &SaberCiphertext) -> Polynomial {
        let params = &self.params;
        let ntt_tables = &self.ntt_precalculated;
        let s = &keypair.secret_key; //In Z_q
        let u = &encapsulated.u;     //In Z_p
        let v = &encapsulated.v;     //In Z_p
        let p = self.p;

        let u_in_q = self.lift(u); //It goes back in Z_q so that we can do the NTT multiplication, which is in Z_q

        let su_raw = s.mul_ntt(params, ntt_tables, &u_in_q); //In Z_q 
        let su = self.round(params, su_raw); //Then returned in Z_p

        let mut m_poly_coeffs = vec![0u64; params.n]; // v - su, in Z_p
        for i in 0..params.n {
            m_poly_coeffs[i] = if v.coeffs[i] >= su.coeffs[i] {
                v.coeffs[i] - su.coeffs[i]
            } else {
                p - (su.coeffs[i] - v.coeffs[i])
            }; 
        }
        let m_poly = Polynomial::new(m_poly_coeffs);
        self.decode(params, &m_poly) //Final decoding, in Z_p
    }


}