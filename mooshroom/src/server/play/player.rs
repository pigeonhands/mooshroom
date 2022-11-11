use mooshroom_core::varint::VarInt;
use mooshroom_macros::Mooshroom;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x16)]
pub struct SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x31)]
pub struct PlayerAbilities {
    pub flags: u8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x39)]
pub struct SynchronizePlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: VarInt,
    pub dismount_vehicle: bool
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x4A)]
pub struct SetHeldItem {
    pub slot: u8,
}
