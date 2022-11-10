use mooshroom_macros::Mooshroom;
use crate::containers::Json;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize,Deserialize, Default)]
pub struct ServerVersion {
    pub name: String,
    pub protocal: i32
}

#[derive(Debug, Clone, Serialize,Deserialize, Default)]
pub struct ServerPlayer {
    pub name: String,
    pub id: uuid::Uuid
}

#[derive(Debug, Clone, Serialize,Deserialize, Default)]
pub struct ServerPlayers {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<ServerPlayers>
}

#[derive(Debug, Clone, Serialize,Deserialize, Default)]
pub struct StatusBody {
    pub version: ServerVersion,

    pub favicon: String,
    #[serde(rename="previewsChat")]
    pub previews_chat: String,
    #[serde(rename="enforcesSecureChat")]
    pub enforces_secure_chat: String
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct StatusResponse{
    pub response: Json<StatusBody>
}
