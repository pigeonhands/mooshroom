use mooshroom_core::{
    error::{MoshroomError, Result},
    io::{MooshroomReadable, MooshroomWritable},
    varint::VarInt,
};
use mooshroom_macros::Mooshroom;

use crate::containers::TOption;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct SignatureData {
    pub timestamp: u64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct LoginStart {
    pub name: String,
    pub sig_data: TOption<SignatureData>,
    pub player_uuid: TOption<uuid::Uuid>,
}
