use crate::{BFV, plaintext::BFVPlaintext};
use simple_ring::{Polynomial, generate_cbd_sample, generate_uniform_polynomial, generate_small_sample, RingParams};

#[derive(Debug, Clone)]
pub struct BFVCiphertext {
    pub c0: Polynomial,
    pub c1: Polynomial,
}


impl BFV {
    //Utils :

    //just return the noise threshold, knowing it's around Delta/2
    pub fn noise_threshold(&self) -> u64 {
        let delta = self.params.q / self.t;
        delta / 2
    }

    /*Estimate noise magnitude after decryption.
    # Warning
    This method reveals information about the secret key and plaintext (obviously, because you HAVE to give the secret key).
    Use ONLY for educational debugging in trusted environments. 
     */
    pub fn estimate_noise(&self, secret_key: &Polynomial, params: &RingParams, ciphertext: &BFVCiphertext) -> u64 {
        let c1s = ciphertext.c1.mul_ntt(params, &self.ntt_precalculated, secret_key);
        let m_prime = ciphertext.c0.sub(params, &c1s);
        
        let mut max_noise = 0u64;
        let q = params.q as i128;
        let delta = (params.q / self.t) as i128; 
        
        for &coeff in m_prime.coeffs.iter() {
            let centered = if coeff as i128 > q / 2 { 
                coeff as i128 - q 
            } else { 
                coeff as i128 
            };
            let remainder = centered.abs() % delta;
            let noise = remainder.min(delta - remainder) as u64;
            if noise > max_noise { max_noise = noise; }
        }
        max_noise
    }


    //The key generation. For informations about math formula, please see /docs/simple-bfv

    pub fn generate_public_b(&self, a: &Polynomial, s: &Polynomial) -> Polynomial { 
        let params = &self.params;
        let ntt_tables = &self.ntt_precalculated;        
        let e = generate_cbd_sample(params.n, self.eta);
        let e = e.to_poly(params.q);
        let mul = a.mul_ntt(params, ntt_tables, &s);
        let b = mul.sum(params, &e);

        b
    }


    pub fn generate_public_a(&self) -> Polynomial {
        let params = &self.params;
        generate_uniform_polynomial(params)
    }


    pub fn generate_secret_key(&self)  -> Polynomial {
        let params = &self.params;
        let s = generate_small_sample(params);
        s.to_poly(params.q)
    }


    //Encryption & Decryption


    pub fn encrypt(&self, message: &BFVPlaintext, public_a: &Polynomial, public_b: &Polynomial) -> BFVCiphertext { //The encryption. Same, you'll find the explanation in /docs/simple-bfv
        let params = &self.params;    
        assert_eq!(
            message.plain.coeffs.len(),
            params.n,
            "Plaintext degree mismatch: expected {}, got {}",
            params.n, message.plain.coeffs.len()
        );
        
        assert_eq!(
            public_a.coeffs.len(), params.n,
            "Public key a(x) degree mismatch"
        );
        assert_eq!(
            public_b.coeffs.len(), params.n,
            "Public key b(x) degree mismatch"
        );

        let ntt_tables = &self.ntt_precalculated;
        
        let u = generate_small_sample(params);
        let u = u.to_poly(params.q);

        let e1 = generate_cbd_sample(params.n, self.eta);
        let e1 = e1.to_poly(params.q);

        let e2 = generate_cbd_sample(params.n, self.eta);
        let e2 = e2.to_poly(params.q);
        
        let delta = params.q / self.t;
        
        let bu = public_b.mul_ntt(params, ntt_tables, &u);
        let delta_m = message.plain.scale(params, delta);
        let c0_temp = bu.sum(params, &e1);
        let c0 = c0_temp.sum(params, &delta_m);
        
        let au = public_a.mul_ntt(params, ntt_tables, &u);
        let c1 = au.sum(params, &e2);

        BFVCiphertext { c0, c1 }
    }



    pub fn backend_decrypt(&self, ciphertext: &BFVCiphertext, secret_key: &Polynomial) -> Polynomial { //The backend decryption. Same, you'll find the explanation at /docs/simple-bfv
        let params = &self.params;
        assert_eq!(
            ciphertext.c0.coeffs.len(), params.n,
            "Ciphertext c0 degree mismatch"
        );
        assert_eq!(
            ciphertext.c1.coeffs.len(), params.n,
            "Ciphertext c1 degree mismatch"
        );
        assert_eq!(
            secret_key.coeffs.len(), params.n,
            "Secret key degree mismatch"
        );
        let ntt_tables = &self.ntt_precalculated;
        let c1s = ciphertext.c1.mul_ntt(params, ntt_tables, secret_key);
        let m_prime = ciphertext.c0.sub(params, &c1s);
            
        let mut coeffs = vec![0u64; params.n];
        let q = params.q as i128;
        let t = self.t as i128;
        for i in 0..params.n {
            let raw = m_prime.coeffs[i] as i128;
            

            let val = if raw > q / 2 { raw - q } else { raw };

            let decoded = ((val * t + q / 2).div_euclid(q) % t + t) % t;

            coeffs[i] = decoded as u64;
        }
        
            
            Polynomial::new(coeffs)
    }


    pub fn decrypt(&self, ciphertext: &BFVCiphertext, secret_key: &Polynomial) -> String { //The decryption. Same, you'll find the explanation at /docs/simple-bfv
        let params = &self.params;
        assert_eq!(
            ciphertext.c0.coeffs.len(), params.n,
            "Ciphertext c0 degree mismatch"
        );
        assert_eq!(
            ciphertext.c1.coeffs.len(), params.n,
            "Ciphertext c1 degree mismatch"
        );
        assert_eq!(
            secret_key.coeffs.len(), params.n,
            "Secret key degree mismatch"
        );
        let ntt_tables = &self.ntt_precalculated;
        let c1s = ciphertext.c1.mul_ntt(params, ntt_tables, secret_key);
        let m_prime = ciphertext.c0.sub(params, &c1s);
            
        let mut coeffs = vec![0u64; params.n];
        let q = params.q as i128;
        let t = self.t as i128;
        for i in 0..params.n {
            let raw = m_prime.coeffs[i] as i128;
            

            let val = if raw > q / 2 { raw - q } else { raw };

            let decoded = ((val * t + q / 2).div_euclid(q) % t + t) % t;

            coeffs[i] = decoded as u64;
        }
        
            
            BFVPlaintext { plain: Polynomial::new(coeffs), len: params.n }.decode()
    }

    //Homomorphic operations

    pub fn sum_ciphertexts(&self, first_cipher: BFVCiphertext, second_cipher: BFVCiphertext) -> BFVCiphertext { //Function to sum two ciphertexts
        let params = &self.params;
        let new_c0 = first_cipher.c0.sum(params, &second_cipher.c0); 
        let new_c1 = first_cipher.c1.sum(params, &second_cipher.c1);
        
        BFVCiphertext { c0: new_c0, c1: new_c1 }
    }



    pub fn sum_ciphertext_and_plaintext(&self, ciphertext: &BFVCiphertext, plaintext: &BFVPlaintext) -> BFVCiphertext { //Function to sum a ciphertext and a plaintext
        let params = &self.params;
        let delta = params.q / self.t; 
        let delta_m = plaintext.plain.scale(params, delta); //We first need to scale the plaintext coeffs with delta
        let new_c0 = ciphertext.c0.sum(params, &delta_m); //Then we can sum, but only on c0, not on c1
        let new_c1 = ciphertext.c1.clone();

        BFVCiphertext { c0: new_c0, c1: new_c1 }
    }

    pub fn mul_ciphertext_and_plaintext(&self, ciphertext: &BFVCiphertext, plaintext: &BFVPlaintext) -> BFVCiphertext { //Function for ciphertext * plaintext
        let params = &self.params;
        let ntt_tables = &self.ntt_precalculated;
        let new_c0 = ciphertext.c0.mul_ntt(params, ntt_tables, &plaintext.plain);
        let new_c1 = ciphertext.c1.mul_ntt(params, ntt_tables, &plaintext.plain);

        BFVCiphertext { c0: new_c0, c1: new_c1 }
    }



}










