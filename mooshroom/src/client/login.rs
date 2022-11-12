use mooshroom_macros::Mooshroom;

use crate::shared::SignatureData;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct LoginStart {
    pub name: String,
    pub sig_data: Option<SignatureData>,
    pub player_uuid: Option<uuid::Uuid>,
}
