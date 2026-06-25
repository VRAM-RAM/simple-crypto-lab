#[derive(Debug)]
pub enum SaberError {
    InvalidMac,
    SerializationError(postcard::Error),
}