use mooshroom_core::varint::VarInt;
use mooshroom_macros::{Mooshroom};


#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x02)]
pub struct SpawnExperienceOrb {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: bool,
    pub count: u16
}
