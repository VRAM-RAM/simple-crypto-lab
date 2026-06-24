#[derive(Debug)]
pub enum SaberError {
    InvalidMac,
    FailedToRecoverParams,
}