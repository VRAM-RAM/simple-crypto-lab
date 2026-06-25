use serde::{Deserialize, Serialize};



#[derive(Clone, Serialize, Deserialize)]
pub struct RingParams {
    pub n: usize,
    pub q: u64,
    pub omega: u64,
}

impl RingParams {
    pub fn new(n: usize, q:u64, omega: u64) -> Self {
        assert!(n.is_power_of_two(), "NTT requires n to be a power of 2, got {} ! For explanations, please read '/docs/simple-ring.pdf", n);
        Self { n, q, omega }
    }
}