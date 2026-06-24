/*
When encapsulating a key, it is useful that the peer knows the used parameters.
*/

use simple_ring::{RingParams};
use crate::Saber;

#[derive(Clone)]
pub struct SaberExportated {
    pub params: RingParams,
    pub eta: usize,
    pub p: u64
}

impl SaberExportated {
    pub fn new(params: RingParams, eta: usize, p: u64) -> Self {
        Self { params, eta, p }
    }

    pub fn to_saber(&self) -> Saber {
        Saber::new(self.params.clone(), self.eta, self.p)
    }
}

impl Saber {
    pub fn to_export(&self) -> SaberExportated {
        SaberExportated::new(self.params.clone(), self.eta, self.p)
    }
}