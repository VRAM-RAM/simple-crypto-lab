

#[derive(Clone)]
pub struct RingParams {
    pub n: usize,
    pub q: u64,
    pub omega: u64,
}

impl RingParams {
    pub fn new(n: usize, q:u64, omega: u64) -> Self {
        Self { n, q, omega }
    }
}