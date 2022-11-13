use mooshroom_core::varint::VarInt;
use mooshroom_macros::{Mooshroom, MooshroomCollection};

use crate::types::Chat;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct Disconnect(Chat);

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x01)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x02)]
pub struct LoginSuccess {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x03)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Debug, Clone, MooshroomCollection)]
pub enum LoginStage {
    Disconnect(Disconnect),
    SetCompression(SetCompression),
    Success(LoginSuccess),
}
