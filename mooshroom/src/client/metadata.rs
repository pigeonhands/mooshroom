use mooshroom_macros::Mooshroom;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x12)]
pub struct KeepAliveResponse(pub i64);
