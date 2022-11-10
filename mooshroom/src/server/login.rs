use mooshroom_core::{
    error::{MoshroomError, Result},
    io::{MooshroomReadable, MooshroomWritable},
    varint::VarInt,
};
use mooshroom_macros::Mooshroom;

use crate::containers::TOption;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Properties {
    pub name: String,
    pub value: String,
    pub signature: TOption<String>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(2)]
pub struct LoginSuccess {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub player_uuid: TOption<uuid::Uuid>,
    pub properties: Vec<Properties>,
}
