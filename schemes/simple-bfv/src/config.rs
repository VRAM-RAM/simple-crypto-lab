use simple_ring::{RingParams, ntt::{NTTprecaculated, precalculate}};


#[derive(Clone)]
pub struct BFV {
    pub params: RingParams,
    pub t: u64,
    pub eta: usize,
    pub ntt_precalculated: NTTprecaculated,
}

impl BFV {
    pub fn for_test() -> Self {
        let params = RingParams { n:64, q:786_433, omega: 368299};
        Self { params: params.clone(), t: 256, eta: 2, ntt_precalculated: precalculate(&params) } 
    }

    pub fn for_medium() -> Self {
        let params = RingParams { n: 1024, q: 2572289, omega:  457185};
        Self { params: params.clone(), t: 256, eta: 4, ntt_precalculated: precalculate(&params) } 
    }

    pub fn for_large() -> Self {
        let params = RingParams { n: 4096, q: 512032769, omega:  431951560};
        Self { params: params.clone(), t: 512, eta: 8, ntt_precalculated : precalculate(&params) } 
    }

} 

