use simple_ring::{RingParams, ntt::{NTTprecaculated, precalculate}, find_valid_omega };


#[derive(Clone)]
pub struct Saber {
    pub params: RingParams,
    pub eta: usize,
    pub ntt_precalculated: NTTprecaculated,
    pub p: u64,
    pub t: u64,
}

impl Saber {
    pub fn recommended() -> Self {
        let params = RingParams { n:1024, q:12289, omega: find_valid_omega(1024, 12289)};
        Self { params: params.clone(), eta: 16, ntt_precalculated: precalculate(&params), p:64, t:2 } 
    }

    pub fn new(params: RingParams, eta: usize, p: u64, t:u64) -> Self {
        Self {params: params.clone(), eta, ntt_precalculated: precalculate(&params), p , t}
    }

} 

