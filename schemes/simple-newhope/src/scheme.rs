
use crate::{NewHope};
use simple_ring::{forward_ntt, Polynomial, generate_cbd_sample, generate_then_shake, generate_small_sample, RingParams};

pub struct EncapsulatedTuple {
    pub u: Polynomial,
    pub v: Polynomial,
}



impl NewHope {
    //The key encapsulation /docs/simple-newhope

    pub fn key_encapsulation(&self) -> Polynomial {
        todo!("Add the Encoding");
        let params = &self.params;
        let ntt_tables = &self.ntt_precalculated;
        let n = params.n;
        let q = params.q;

        let a_ntt = generate_then_shake(params); //We consider the generated coefficients as in the NTT domain
        let a_ntt = a_ntt.to_poly(q);

        let s = generate_cbd_sample(n, self.eta);
        let s = s.to_poly(q);

        let e = generate_cbd_sample(n, self.eta);
        let e = e.to_poly(q);

        let s_ntt = forward_ntt(params, &s, ntt_tables);
        let e_ntt = forward_ntt(params, &e, ntt_tables);
        
        let mut b_ntt = Polynomial::zeros(params.n);

        for i in 0..params.n {
            b_ntt.coeffs[i] = ((s_ntt.coeffs[i] as u128 * a_ntt.coeffs[i] as u128) % params.q as u128) as u64;
        }
        let result = b_ntt.sum(params, &e_ntt);
        
        result
    }


    pub fn key_treatment(&self, encapsulated_key: Polynomial) -> EncapsulatedTuple {
        todo!("Finish it...");
        let params = &self.params;
        let ntt_tables = &self.ntt_precalculated;
        let n = params.n;
        let q = params.q;

        let s = generate_cbd_sample(n, self.eta);
        let s = s.to_poly(q);

        let e1 = generate_cbd_sample(n, self.eta);
        let e1 = e1.to_poly(q);

        let e2 = generate_cbd_sample(n, self.eta);
        let e2 = e2.to_poly(q);

        let a_ntt = generate_then_shake(params); //We also consider the generated coefficients as in the NTT domain
        let a_ntt = a_ntt.to_poly(q);
    }


}










