use mooshroom_core::{
    io::MooshroomReadProto,
    varint::{VarInt, VarLong},
};
use mooshroom_macros::Mooshroom;

use super::{nbt, population::WorldPosition};
use crate::{
    core::primitives::{Identifier, Position, Vec3},
    types::Chat,
};

pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Angle(u8);
impl Angle {
    pub fn to_deg(&self) -> f32 {
        (self.0 as f32) / (256.0 / 360.0)
    }
}


#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x0b)]
pub struct ChangeDifficulty {
    pub difficulty: u8,
    pub difficulty_locked: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x09)]
pub struct BlockUpdate {
    pub location: Position,
    pub block_id: VarInt,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x1A)]
pub struct EntityEvent {
    pub entity_id: i32,
    pub status: u8,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x1d)]
pub struct GameEvent {
    pub event_id: u8,
    pub value: f32,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x1F)]
pub struct InitializeWorldBorder {
    pub x: f64,
    pub z: f64,
    pub old_diamiter: f64,
    pub new_diamiter: f64,
    pub speed: VarLong,
    pub portal_teleport_boundry: VarInt,
    pub warning_blocks: VarInt,
    pub warning_timer: VarInt,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct DeathLocation {
    pub dimention_name: Identifier,
    pub location: Position,
}

// calculated from ((blockX & 15) << 4) | (blockZ & 15)
#[derive(Debug, Clone, Default, Mooshroom)]
pub struct PackedXZ(i8);

impl PackedXZ {
    pub fn unpack(&self) -> (i8,i8) {
        let x = (self.0 >> 4) & 15;
        let z = self.0 & 15;
        (x,z)
    }
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct BlockEntity {
    pub xz: PackedXZ,
    pub y: u16,
    pub block_type: VarInt,
    pub data: nbt::NptCompound,
}
pub type BitSet = Vec<u64>;
// type LightArray = [u8;2048];

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct LightingData {
    pub trust_edges: bool,
    //pub sky_light_mask: BitSet,
    //pub block_light_mask: BitSet,
    //pub empty_sky_mask: BitSet,
    //pub empty_block_mask: BitSet,
    //pub sky_light_arrays: Vec<LightArray>,
    //pub block_light_n: VarLong,
    //pub block_light_arrays: Vec<LightArray>,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x21)]
pub struct ChunkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: nbt::NptCompound,
    pub data: Vec<u8>,
    pub entity_blocks: Vec<BlockEntity>,
    pub lighting: LightingData,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x22)]
pub struct WorldEvent {
    pub event_id: i32,
    pub location: Position,
    pub disable_relitive_volume: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x23)]
pub struct Particle {
    pub particle_id: VarInt,
    pub long_distance: bool,
    pub location: WorldPosition,
    pub offset: Vec3<f32>,
    pub max_speed: f32,
    pub particle_count: i32,
    //TODO: data
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x24)]
pub struct UpdateLight {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub lighting: LightingData,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[repr(i8)]
pub enum GameMode {
    #[default]
    None = -1,
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x25)]
pub struct LoginPlay {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub gamemode: GameMode,
    pub previous_gamemode: GameMode,
    pub dimention_names: Vec<String>,
    pub npt: nbt::NptCompound,
    pub dimention_type: Identifier,
    pub dimention_name: Identifier,
    pub hashed_seed: u64,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_infomation: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct PreviousMessage {
    pub sender: uuid::Uuid,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[repr(i32)]
#[value_type(VarInt)]
pub enum FilterType {
    #[default]
    PassThrough = 0,
    FullyFiltered = 1,
    PartiallyFiltered = 2,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x33)]
pub struct PlayerChatMessage {
    pub message_signature: Option<Vec<u8>>,
    pub sender: uuid::Uuid,
    pub header_signature: Option<Vec<u8>>,
    pub plain_message: String,
    pub formatted_message: Option<Chat>,
    pub timestamp: i64,
    pub salt: i64,
    pub previous_messages: Vec<PreviousMessage>,
    pub unsigned_content: Option<Chat>,
    pub filter_type: FilterType,

    #[read(read_filter_mask, filter_type)]
    pub filter_mask: Option<BitSet>,

    pub chat_type: VarInt,
    pub network_name: Chat,
    pub target_network_name: Option<Chat>,
}
fn read_filter_mask<const PV: usize>(
    reader: &mut impl std::io::Read,
    filter_type: &FilterType,
) -> crate::core::error::Result<Option<BitSet>> {
    let r = match filter_type {
        FilterType::FullyFiltered => Some(BitSet::read_proto::<PV>(reader)?),
        _ => None,
    };
    Ok(r)
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x40)]
pub struct UpdateSectionBlocks {
    pub chunk_section_position: i64,
    pub suppress_light_updates: bool,
    pub blocks: Vec<VarLong>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x4B)]
pub struct SetCenterChunk {
    pub x: VarInt,
    pub y: VarInt,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x4c)]
pub struct SetRenderDistance(VarInt);

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x5A)]
pub struct SetSimulationDistance(VarInt);

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x5C)]
pub struct UpdateTime {
    pub world_age: u64,
    pub time_of_day: u64,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x60)]
pub struct SoundEffect {
    pub sound_id: VarInt,
    pub sound_category: VarInt,
    pub position: Vec3<i32>,
    pub volume: f32,
    pub pitch: f32,
    pub speed: i64,
}
