use mooshroom_macros::Mooshroom;
use serde::{Deserialize, Serialize};

use crate::containers::Json;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerPlayer {
    pub name: String,
    pub id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerPlayers {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<ServerPlayers>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusBody {
    pub version: ServerVersion,

    pub favicon: String,
    #[serde(rename = "previewsChat")]
    pub previews_chat: Option<bool>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct StatusResponse {
    pub response: Json<StatusBody>,
}
