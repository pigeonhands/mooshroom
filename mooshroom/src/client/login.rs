use mooshroom_core::{error::Result, io::MooshroomReadProto};
use mooshroom_macros::Mooshroom;

use crate::shared::SignatureData;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x0)]
pub struct LoginStart {
    pub name: String,
    pub sig_data: Option<SignatureData>,
    pub player_uuid: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct SignatureInfo {
    pub salt: i64,
    pub signature: Vec<u8>,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x01)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub has_verify_token: bool,
    #[read(read_verify_token, has_verify_token)]
    pub verify_token: Option<Vec<u8>>,
    #[read(read_signature, has_verify_token)]
    pub signature: Option<SignatureInfo>,
}

fn read_verify_token<const PV: usize>(
    reader: &mut impl std::io::Read,
    has_verify_token: &bool,
) -> Result<Option<Vec<u8>>> {
    if *has_verify_token {
        Ok(Some(Vec::read_proto::<PV>(reader)?))
    } else {
        Ok(None)
    }
}
fn read_signature<const PV: usize>(
    reader: &mut impl std::io::Read,
    has_verify_token: &bool,
) -> Result<Option<SignatureInfo>> {
    let r = if *has_verify_token {
        None
    } else {
        Some(SignatureInfo {
            salt: i64::read_proto::<PV>(reader)?,
            signature: Vec::read_proto::<PV>(reader)?,
        })
    };
    Ok(r)
}
