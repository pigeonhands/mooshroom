use mooshroom_core::{
    io::{
        MooshroomPacket,
        MooshroomReadProto,
        MooshroomReadable,
        MooshroomWritable,
        MooshroomWriteProto,
    },
    primitives::{Identifier, Vec3},
    varint::VarInt,
};
use mooshroom_macros::Mooshroom;

use super::{
    crafting::Slot,
    nbt,
    world::{Angle},
    entity
};
use crate::shared::SignatureData;

pub type WorldPosition = Vec3<f64>;


#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x0)]
pub struct SpawnEntity {
    pub entity_id: entity::EntityType,
    pub entity_uuid: uuid::Uuid,
    pub entity_type: VarInt,
    pub position: WorldPosition,
    pub pitch: Angle,
    pub yaw: Angle,
    pub data: VarInt,
    pub velocity: Vec3<i16>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x02)]
pub struct SpawnPlayer {
    pub entity_id: VarInt,
    pub player_uuid: uuid::Uuid,
    pub position: WorldPosition,
    pub yaw: Angle,
    pub pitch: Angle,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[repr(u8)]
pub enum Animation {
    #[default]
    SwingMainArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    SwingOffhand = 3,
    CriticalEffect=4,
    MagicCriticalEfect = 5
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x03)]
pub struct EntityAnimation {
    pub entity_id: VarInt,
    pub animation: Animation,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x28)]
pub struct UpdateEntityPosition {
    pub entity_id: VarInt,
    pub delta: Vec3<i16>,
    pub on_ground: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x29)]
pub struct UpdateEntityPositionAndRotation {
    pub entity_id: VarInt,
    pub delta: Vec3<i16>,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x2a)]
pub struct UpdateEntityRotation {
    pub entity_id: VarInt,
    pub yaw: Angle,
    pub pitch: Angle,
    pub is_overlay: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x3B)]
pub struct RemoveEntity {
    pub entities: Vec<VarInt>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AddPlayer {
    pub name: String,
    pub properties: Vec<PlayerProperty>,
    pub gamemode: super::world::GameMode, //Possably should be varInt?
    pub ping: VarInt,
    pub display_name: Option<String>,
    pub signature_data: Option<SignatureData>,
}

#[derive(Debug, Clone, Default)]
pub struct ActionFor<T> {
    pub uuid: uuid::Uuid,
    pub action: T,
}

impl<const PV: usize, T> MooshroomReadable<PV> for ActionFor<T>
where
    T: MooshroomReadable<PV>,
{
    fn read(reader: &mut impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        Ok(Self {
            uuid: uuid::Uuid::read_proto::<PV>(reader)?,
            action: T::read_proto::<PV>(reader)?,
        })
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for ActionFor<T>
where
    T: MooshroomWritable<PV>,
{
    fn write(&self, writer: &mut impl std::io::Write) -> mooshroom_core::error::Result<()> {
        self.uuid.write_proto::<PV>(writer)?;
        self.action.write_proto::<PV>(writer)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub enum PlayerAction {
    AddPlayer(Vec<ActionFor<AddPlayer>>),
    UpdateGamemode(Vec<ActionFor<VarInt>>),
    UpdateLatency(Vec<ActionFor<VarInt>>),
    UpdateDisplayName(Vec<ActionFor<Option<String>>>),
    RemovePlayer,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInfo(PlayerAction);
impl<const PV: usize> MooshroomPacket<PV> for PlayerInfo {
    const PACKET_ID: VarInt = VarInt(0x37);
}

impl<const PV: usize> MooshroomReadable<PV> for PlayerInfo {
    fn read(reader: &mut impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let action = VarInt::read_proto::<PV>(reader)?;
        let ser_action = match action.0 {
            0 => PlayerAction::AddPlayer(Vec::read_proto::<PV>(reader)?),
            1 => PlayerAction::UpdateGamemode(Vec::read_proto::<PV>(reader)?),
            2 => PlayerAction::UpdateLatency(Vec::read_proto::<PV>(reader)?),
            3 => PlayerAction::UpdateDisplayName(Vec::read_proto::<PV>(reader)?),
            4 => PlayerAction::RemovePlayer,
            _ => PlayerAction::Unknown,
        };
        Ok(Self(ser_action))
    }
}

impl<const PV: usize> MooshroomWritable<PV> for PlayerInfo {
    fn write(&self, writer: &mut impl std::io::Write) -> mooshroom_core::error::Result<()> {
        match &self.0 {
            PlayerAction::AddPlayer(p) => {
                VarInt(0).write_proto::<PV>(writer)?;
                p.write_proto::<PV>(writer)?;
            }
            PlayerAction::UpdateGamemode(p) => {
                VarInt(1).write_proto::<PV>(writer)?;
                p.write_proto::<PV>(writer)?;
            }
            PlayerAction::UpdateLatency(p) => {
                VarInt(2).write_proto::<PV>(writer)?;
                p.write_proto::<PV>(writer)?;
            }
            PlayerAction::UpdateDisplayName(p) => {
                VarInt(3).write_proto::<PV>(writer)?;
                p.write_proto::<PV>(writer)?;
            }
            PlayerAction::RemovePlayer => {
                VarInt(4).write_proto::<PV>(writer)?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x3f)]
pub struct SetHeadRotation {
    pub entity_id: VarInt,
    pub head_yaw: Angle,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x52)]
pub struct SetEntityVelocity {
    pub entity_id: VarInt,
    pub velocity: Vec3<i16>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Equipment {
    pub slot: u8,
    pub item: Slot,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x51)]
pub struct LinkEntities {
    pub attached_entity: i32,
    pub holding_entity_id: i32,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x53)]
pub struct SetEquipment {
    pub entity_id: VarInt,
    pub equipment: Equipment,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x57)]
pub struct SetPassengers {
    pub entity_id: VarInt,
    pub passengers: Vec<VarInt>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x62)]
pub struct SystemChatMessage {
    pub json: String,
    pub is_overlay: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x66)]
pub struct TeleportEntity {
    pub entity_id: VarInt,
    pub location: WorldPosition,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Modifier {
    pub uuid: uuid::Uuid,
    pub amount: f64,
    pub operaion: u8,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AttributeProperty {
    pub key: Identifier,
    pub value: f64,
    pub modifiers: Vec<Modifier>,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x68)]
pub struct UpdateAttributes {
    pub entity_id: VarInt,
    pub properties: Vec<AttributeProperty>,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x69)]
pub struct EntityEffect {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
    pub aplifier: i8,
    pub duration: VarInt,
    pub flags: i8,
    pub factor_codec: Option<nbt::NptCompound>,
}
