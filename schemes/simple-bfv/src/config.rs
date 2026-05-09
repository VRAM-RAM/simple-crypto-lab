use simple_ring::{RingParams, find_valid_omega, ntt::{NTTprecaculated, precalculate}};

#[derive(Clone)]
pub struct BFV {
    pub params: RingParams,
    pub t: u64,
    pub eta: usize,
    pub ntt_precalculated: NTTprecaculated,
}

impl BFV {
    pub fn for_test() -> Self {
        let params = RingParams { n:64, q:786_433, omega:find_valid_omega(64, 786_433) };
        Self { params: params.clone(), t: 256, eta: 2, ntt_precalculated: precalculate(&params) } 
    }

    pub fn for_medium() -> Self {
        let params = RingParams { n: 1028, q:16_760_833, omega: find_valid_omega(1028, 16_760_833) };
        Self { params: params.clone(), t: 256, eta: 4, ntt_precalculated: precalculate(&params) } 
    }

    pub fn for_large() -> Self {
        let params = RingParams { n: 4096, q: 5234689, omega: find_valid_omega(4096, 5234689) };
        Self { params: params.clone(), t: 512, eta: 8, ntt_precalculated : precalculate(&params) } 
    }

} 

