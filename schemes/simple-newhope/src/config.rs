use simple_ring::{RingParams, ntt::{NTTprecaculated, precalculate}, find_valid_omega };


#[derive(Clone)]
pub struct NewHope {
    pub params: RingParams,
    pub eta: usize,
    pub ntt_precalculated: NTTprecaculated,
}

impl NewHope {
    pub fn recommended() -> Self {
        let params = RingParams { n:1024, q:12289, omega: find_valid_omega(1024, 12289)};
        Self { params: params.clone(), eta: 16, ntt_precalculated: precalculate(&params) } 
    }

    pub fn new(params: RingParams, eta: usize) -> Self {
        Self {params: params.clone(), eta, ntt_precalculated: precalculate(&params) }
    }

} 

