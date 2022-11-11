use mooshroom_core::varint::VarInt;
use mooshroom_macros::{Mooshroom, MooshroomCollection};

use crate::containers::TOption;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Properties {
    pub name: String,
    pub value: String,
    pub signature: TOption<String>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct Disconnect(String);

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(2)]
pub struct LoginSuccess {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub properties: Vec<Properties>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(3)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Debug, Clone, MooshroomCollection)]
pub enum LoginStage {
    Disconnect(Disconnect),
    SetCompression(SetCompression),
    Success(LoginSuccess),
}
