use simple_ring::{RingParams, ntt::{NTTprecaculated, precalculate}, find_valid_omega };


#[derive(Clone)]
pub struct Saber {
    pub params: RingParams,
    pub eta: usize,
    pub ntt_precalculated: NTTprecaculated,
    pub p: u64,
}

/*
In the real saber algorithm, these are the parameters :
| Scheme         | (l) | (n) |      (q) |      (p) |   (t) | (eta) | Post-quantum security | Failure probability | NIST level |
| -------------- | --  | --  | -------: | -------: | ----  | ----  | --------------------   | ------------------ | ---------  |
| **LightSaber** |   2 | 256 | (2^{13}) | (2^{10}) | (2^3) |     5 |             (2^{107}) |          (2^{-120}) |          1 |
| **Saber**      |   3 | 256 | (2^{13}) | (2^{10}) | (2^4) |     4 |             (2^{172}) |          (2^{-136}) |          3 |
| **FireSaber**  |   4 | 256 | (2^{13}) | (2^{10}) | (2^6) |     3 |             (2^{236}) |          (2^{-165}) |          5 |

Here, I simplified
*/

impl Saber {
    pub fn for_test() -> Self {
        let params = RingParams { n:1024, q:12289, omega: find_valid_omega(1024, 12289)};
        Self { params: params.clone(), eta: 2, ntt_precalculated: precalculate(&params), p:256 } 
    }
    
    pub fn light() -> Self {
        let params = RingParams { n:256, q:12289, omega: find_valid_omega(256, 12289)};
        Self { params: params.clone(), eta: 3, ntt_precalculated: precalculate(&params), p:2048 } 
    }

    pub fn classic() -> Self {
        let params = RingParams { n: 256, q: 12289, omega: find_valid_omega(256, 12289)};
        Self { params: params.clone(), eta: 4, ntt_precalculated: precalculate(&params), p:2048 } //Only eta changes and grows
    }

    pub fn fire() -> Self {
        let params = RingParams { n: 256, q: 12289, omega: find_valid_omega(256, 12289)};
        Self { params: params.clone(), eta: 5, ntt_precalculated: precalculate(&params), p:2048 } 
    }

    pub fn new(params: RingParams, eta: usize, p: u64) -> Self {
        Self {params: params.clone(), eta, ntt_precalculated: precalculate(&params), p}
    }

} 

