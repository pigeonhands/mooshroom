use mooshroom_core::{varint::VarInt, primitives::{Position, Identifier}, io::{MooshroomReadable, MooshroomReadProto, MooshroomWritable, MooshroomWriteProto}};
use mooshroom_macros::Mooshroom;

use crate::{containers::TOption, types::Chat};

use super::{world::Angle, crafting::Slot};

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
#[packet_id(0x31)]
pub struct PlayerAbilities {
    pub flags: u8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x34)]
pub struct EndCombat {
    pub duration: VarInt,
    pub entity_id: i32
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
    pub dismount_vehicle: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct DeathLocation {
    pub dimention: Identifier,
    pub location: Position
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id( 0x3e)]
pub struct Respawn {
    pub dimention: Identifier,
    pub dimention_name: Identifier,
    pub hash_seed: i64,
    pub gamemode: u8,
    pub previous_gamemode: u8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub copy_metadata: bool,
    pub death_location: TOption<DeathLocation>
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
    pub angle: Angle
}


#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x54)]
pub struct SetExperience {
    pub experience_bar: f32,
    pub level: VarInt,
    pub total_experience: VarInt
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x55)]
pub struct SetHealth {
    pub health: f32,
    pub food: VarInt,
    pub food_saturation: f32
}


#[derive(Debug, Clone, Default)]
pub struct AdvancementDisplay{
    pub title: Chat,
    pub description: Chat,
    pub icon: Slot,
    pub frame_type: VarInt,
    pub flags: i32,
    pub backdround_texture: TOption<Identifier>,
    pub x_coord: f32,
    pub y_coord: f32,
}

impl<const PV:usize> MooshroomReadable<PV> for AdvancementDisplay {
    fn read(mut reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let title = Chat::read_proto::<PV>(&mut reader)?;
        let description = Chat::read_proto::<PV>(&mut reader)?;
        let icon = Slot::read_proto::<PV>(&mut reader)?;
        let frame_type = VarInt::read_proto::<PV>(&mut reader)?;
        let flags = i32::read_proto::<PV>(&mut reader)?;
        let backdround_texture = if (flags & 0x01) != 0 {
            Some(Identifier::read_proto::<PV>(&mut reader)?)
        }else{
            None
        }.into();

        Ok(Self{
            title,
            description,
            icon,
            frame_type,
            flags,
            backdround_texture,
            x_coord: f32::read_proto::<PV>(&mut reader)?,
            y_coord: f32::read_proto::<PV>(&mut reader)?,
        })
    }
}

impl<const PV:usize> MooshroomWritable<PV> for AdvancementDisplay {
    fn write(&self, mut writer: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        self.title.write_proto::<PV>(&mut writer)?;
        self.description.write_proto::<PV>(&mut writer)?;
        self.icon.write_proto::<PV>(&mut writer)?;
        self.frame_type.write_proto::<PV>(&mut writer)?;
        (self.flags & self.backdround_texture.is_some() as i32).write_proto::<PV>(&mut writer)?;
        self.backdround_texture.write_proto::<PV>(&mut writer)?;
        self.x_coord.write_proto::<PV>(&mut writer)?;
        self.y_coord.write_proto::<PV>(&mut writer)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AdvancementCriteria {
    pub achieved: bool,
    pub date_of_achieving: i64
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct ProgressMapping{
    pub key: Identifier,
    pub value: AdvancementCriteria,
}

type AdvancementRequirement = Vec<String>;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Advancement {
    pub parent_id: TOption<Identifier>,
    pub display_data: TOption<AdvancementDisplay>,
    pub criteria: Vec<Identifier>,
    pub requirements: Vec<AdvancementRequirement>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AdvancementMapping{
    pub key: Identifier,
    pub value: Advancement,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x67)]
pub struct UpdateAdvancements {
    pub clear: bool,
    pub advancement_mapping: Vec<AdvancementMapping>,
    pub identifiers: Vec<Identifier>,
    pub progress_mapping: Vec<ProgressMapping>
}