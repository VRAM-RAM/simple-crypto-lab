#[derive(Debug)]
pub enum SaberError {
    InvalidMac,
    FailedToRecoverParams,
    SerializationError(postcard::Error),
}