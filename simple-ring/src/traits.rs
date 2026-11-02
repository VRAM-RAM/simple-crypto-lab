pub trait RingCompatible {
    fn to_u128(&self) -> u128;
    fn to_i128(&self) -> i128;
    fn from_i128(value: i128) -> Self;
    fn from_u128(value: u128) -> Self;
}