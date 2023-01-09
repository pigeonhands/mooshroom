use std::io;

use mooshroom_core::{
    error::Result,
    io::MooshroomReadProto,
    primitives::{Identifier, Position},
    varint::VarInt,
};
use mooshroom_macros::{Mooshroom, MooshroomBitfield};

use super::{crafting::Slot, world::Angle};
use crate::types::Chat;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x16)]
pub struct SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x19)]
pub struct Disconnect(Chat);

#[derive(Debug, Clone, Default, Mooshroom)]
#[repr(u8)]
#[value_type(VarInt)]
pub enum PlayerAction {
    #[default]
    StartSneeking=0,
    StopSneeking=1,
    LeaveBed=2,
    StartSprinting=3,
    StopSprinting=4,
    StartJumpWithHorse=5,
    StopJumpWithHorse=6,
    OpenHorseInventory=7,
    StartFLyingWithElytra=8
}
//
//#[derive(Debug, Clone, Default, Mooshroom)]
//#[packet_id(0x1d)]
//pub struct PlayerCommand{
//    pub entity_id: VarInt,
//    pub action: PlayerAction,
//    pub jump_boost: VarInt
//}


#[derive(Debug, Clone, Default, MooshroomBitfield)]
#[value_type(u8)]
pub struct PlayerAbilityFlags {
    #[mask(0x01)]
    pub invunerable: bool,
    #[mask(0x02)]
    pub flying: bool,
    #[mask(0x04)]
    pub allow_flying: bool,
    #[mask(0x08)]
    pub creative_mode: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x31)]
pub struct PlayerAbilities {
    pub flags: PlayerAbilityFlags,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x34)]
pub struct EndCombat {
    pub duration: VarInt,
    pub entity_id: i32,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x35)]
pub struct EnterCombat;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x36)]
pub struct CombatDeath {
    pub player_id: VarInt,
    pub entity_id: VarInt,
    pub message: Chat,
}

#[derive(Debug, Clone, Default, MooshroomBitfield)]
#[value_type(u8)]
pub struct RotationBitFlags {
    #[mask(0x01)]
    pub x_is_relitive: bool,
    #[mask(0x02)]
    pub y_is_relitive: bool,
    #[mask(0x04)]
    pub z_is_relitive: bool,
    #[mask(0x08)]
    pub y_rot_is_relitive: bool,
    #[mask(0x10)]
    pub x_rot_is_relitive: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x39)]
pub struct SynchronizePlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: RotationBitFlags,
    pub teleport_id: VarInt,
    pub dismount_vehicle: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct DeathLocation {
    pub dimention: Identifier,
    pub location: Position,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x3e)]
pub struct Respawn {
    pub dimention: Identifier,
    pub dimention_name: Identifier,
    pub hash_seed: i64,
    pub gamemode: super::world::GameMode,
    pub previous_gamemode: super::world::GameMode,
    pub is_debug: bool,
    pub is_flat: bool,
    pub copy_metadata: bool,
    pub death_location: Option<DeathLocation>,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x4A)]
pub struct SetHeldItem {
    pub slot: u8,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x4D)]
pub struct SetDefaultSpawnPosition {
    pub location: Position,
    pub angle: Angle,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x54)]
pub struct SetExperience {
    pub experience_bar: f32,
    pub level: VarInt,
    pub total_experience: VarInt,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x55)]
pub struct SetHealth {
    pub health: f32,
    pub food: VarInt,
    pub food_saturation: f32,
}

#[derive(Debug, Clone, Default, MooshroomBitfield)]
#[value_type(u8)]
pub struct AdvancementDisplayFlags {
    #[mask(0x01)]
    pub has_background_texture: bool,
    #[mask(0x02)]
    pub show_toast: bool,
    #[mask(0x04)]
    pub hidden: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AdvancementDisplay {
    pub title: Chat,
    pub description: Chat,
    pub icon: Slot,
    pub frame_type: VarInt,
    pub flags: AdvancementDisplayFlags,
    #[read(read_background_texture, flags)]
    pub backdround_texture: Option<Identifier>,
    pub x_coord: f32,
    pub y_coord: f32,
}
fn read_background_texture<const PV: usize>(
    reader: &mut impl io::Read,
    flags: &AdvancementDisplayFlags,
) -> Result<Option<Identifier>> {
    flags
        .has_background_texture
        .then(|| Identifier::read_proto::<PV>(reader))
        .map_or(Ok(None), |r| r.map(Some))
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AdvancementCriteria {
    pub achieved: bool,
    pub date_of_achieving: i64,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct ProgressMapping {
    pub key: Identifier,
    pub value: AdvancementCriteria,
}

type AdvancementRequirement = Vec<String>;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Advancement {
    pub parent_id: Option<Identifier>,
    pub display_data: Option<AdvancementDisplay>,
    pub criteria: Vec<Identifier>,
    pub requirements: Vec<AdvancementRequirement>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AdvancementMapping {
    pub key: Identifier,
    pub value: Advancement,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x67)]
pub struct UpdateAdvancements {
    pub clear: bool,
    // TODO: Fix
    pub advancement_mapping: Vec<AdvancementMapping>,
    pub identifiers: Vec<Identifier>,
    pub progress_mapping: Vec<ProgressMapping>,
}
