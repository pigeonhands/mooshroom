use mooshroom_macros::Mooshroom;
use crate::shared::SignatureData;
use crate::containers::TOption;


#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct LoginStart {
    pub name: String,
    pub sig_data: TOption<SignatureData>,
    pub player_uuid: TOption<uuid::Uuid>,
}
