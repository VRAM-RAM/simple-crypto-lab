use simple_ring::{RingParams, ntt::{NTTprecaculated, precalculate}, find_valid_omega };


#[derive(Clone)]
pub struct Saber {
    pub params: RingParams,
    pub eta: usize,
    pub ntt_precalculated: NTTprecaculated,
    pub p: u64,
}

impl Saber {
    pub fn for_test() -> Self {
        let params = RingParams { n:1024, q:12289, omega: find_valid_omega(1024, 12289)};
        Self { params: params.clone(), eta: 2, ntt_precalculated: precalculate(&params), p:256 } 
    }
    
    pub fn light() -> Self {
        let params = RingParams { n:256, q:12289, omega: find_valid_omega(256, 12289)};
        Self { params: params.clone(), eta: 5, ntt_precalculated: precalculate(&params), p:2048 } 
    }

    pub fn new(params: RingParams, eta: usize, p: u64) -> Self {
        Self {params: params.clone(), eta, ntt_precalculated: precalculate(&params), p}
    }

} 

