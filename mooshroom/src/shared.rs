use mooshroom_macros::Mooshroom;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct SignatureData {
    pub timestamp: u64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}