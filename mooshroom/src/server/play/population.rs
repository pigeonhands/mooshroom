use mooshroom_core::{
    io::{
        MooshroomCollectionProto,
        MooshroomPacket,
        MooshroomReadProto,
        MooshroomReadable,
        MooshroomWritable,
        MooshroomWriteProto,
    },
    primitives::{Identifier, Vec3},
    varint::VarInt,
};
use mooshroom_macros::{Mooshroom, MooshroomBitfield, MooshroomCollection};

use super::{crafting::Slot, entity, nbt, world::Angle};
use crate::shared::SignatureData;

pub type WorldPosition = Vec3<f64>;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x0)]
pub struct SpawnEntity {
    pub entity_id: VarInt,
    pub entity_uuid: uuid::Uuid,
    pub entity_type: entity::EntityType,
    pub position: WorldPosition,
    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,
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
    CriticalEffect = 4,
    MagicCriticalEfect = 5,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x03)]
pub struct EntityAnimation {
    pub entity_id: VarInt,
    pub animation: Animation,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x1A)]
pub struct EntityEvent {
    pub entity_id: i32,
    pub status: u8,
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
pub struct RemoveEntities {
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

#[derive(Debug, Clone, MooshroomCollection)]
pub enum PlayerAction {
    #[id(0)]
    AddPlayer(Vec<ActionFor<AddPlayer>>),
    #[id(1)]
    UpdateGamemode(Vec<ActionFor<VarInt>>),
    #[id(2)]
    UpdateLatency(Vec<ActionFor<VarInt>>),
    #[id(3)]
    UpdateDisplayName(Vec<ActionFor<Option<String>>>),
    #[id(4)]
    RemovePlayer(()),
}

impl Default for PlayerAction {
    fn default() -> Self {
        Self::RemovePlayer(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInfo(PlayerAction);
impl<const PV: usize> MooshroomPacket<PV> for PlayerInfo {
    const PACKET_ID: VarInt = VarInt(0x37);
}

impl<const PV: usize> MooshroomReadable<PV> for PlayerInfo {
    fn read(reader: &mut impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let action = VarInt::read_proto::<PV>(reader)?;
        Ok(Self(PlayerAction::read_one_of_proto::<PV>(action, reader)?))
    }
}

impl<const PV: usize> MooshroomWritable<PV> for PlayerInfo {
    fn write(&self, writer: &mut impl std::io::Write) -> mooshroom_core::error::Result<()> {
        self.0.variant_id_proto::<PV>().write_proto::<PV>(writer)?;
        self.0.write_one_of_proto::<PV>(writer)?;
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
#[packet_id(0x66)]
pub struct TeleportEntity {
    pub entity_id: VarInt,
    pub location: WorldPosition,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[repr(u8)]
pub enum ModifierOperation {
    #[default]
    AddOrSubtract = 0,
    AddOrSubtractPercent = 1,
    Multiply = 2,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct Modifier {
    pub uuid: uuid::Uuid,
    pub amount: f64,
    pub operaion: ModifierOperation,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[repr(i32)]
#[value_type(VarInt)]
pub enum AttributePropertyKey {
    #[default]
    #[id("minecraft:generic.max_health")]
    MaxHealth = 0,
    #[id("minecraft:generic.follow_range")]
    FollowRange = 1,
    #[id("minecraft:generic.knockback_resistance")]
    KnockbackResistance = 2,
    #[id("minecraft:generic.movement_speed")]
    MovementSpeed = 3,
    #[id("minecraft:generic.flying_speed")]
    FlyingSpeed = 4,
    #[id("minecraft:generic.attack_damage")]
    AttackDamage = 5,
    #[id("minecraft:generic.attack_knockback")]
    AttackKnockback = 6,
    #[id("minecraft:generic.attack_speed")]
    AttackSpeed = 7,
    #[id("minecraft:generic.armor")]
    Armor = 8,
    #[id("minecraft:generic.armor_toughness")]
    ArmorToughness = 9,
    #[id("minecraft:generic.luck")]
    Luck = 10,
    #[id("minecraft:zombie.spawn_reinforcements")]
    ZombieSpawnReinforcements = 11,
    #[id("minecraft:horse.jump_strength")]
    HorseJumpStrength = 12,
    #[id("minecraft:generic.reachDistance")]
    ForgeReachDistance = 13,
    #[id("minecraft:forge.swimSpeed")]
    ForgeSwimSpeed = 14,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AttributeProperty {
    pub key: Identifier, //AttributePropertyKey
    pub value: f64,
    pub modifiers: Vec<Modifier>,
}
#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x68)]
pub struct UpdateAttributes {
    pub entity_id: VarInt,
    pub properties: Vec<AttributeProperty>,
}

#[derive(Debug, Clone, Default, MooshroomBitfield)]
#[value_type(u8)]
pub struct EntityEffectFlags {
    #[mask(0x01)]
    pub is_ambient: bool,
    #[mask(0x02)]
    pub show_particles: bool,
    #[mask(0x04)]
    pub show_icon: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x69)]
pub struct EntityEffect {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
    pub aplifier: i8,
    pub duration: VarInt,
    pub flags: EntityEffectFlags,
    pub factor_codec: Option<nbt::NptCompound>,
}
