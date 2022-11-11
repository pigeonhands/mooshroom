use core::num;

use mooshroom_core::io::{MooshroomReadable, MooshroomWritable, MooshroomReadProto, MooshroomPacket, MooshroomWriteProto};
use mooshroom_core::varint::VarInt;
use mooshroom_macros::Mooshroom;
use crate::shared::SignatureData;

use crate::containers::TOption;

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: TOption<String>
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct AddPlayer {
    pub name: String,
    pub properties: Vec<PlayerProperty>,
    pub gamemode: VarInt,
    pub ping: VarInt,
    pub display_name: TOption<String>,
    pub signature_data: TOption<SignatureData>
}

#[derive(Debug, Clone, Default)]
pub struct ActionFor<T> {
    pub uuid: uuid::Uuid,
    pub action: T
}


impl<const PV: usize, T> MooshroomReadable<PV> for ActionFor<T> 
    where T: MooshroomReadable<PV> {
    fn read(mut reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        Ok(
            Self { 
                uuid: uuid::Uuid::read_proto::<PV>(&mut reader)?, 
                action: T::read_proto::<PV>(&mut reader)? 
            }
        )
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for ActionFor<T>
where T: MooshroomWritable<PV> {
    fn write(&self, mut writer: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        self.uuid.write_proto::<PV>(&mut writer)?;
        self.action.write_proto::<PV>(&mut writer)?;
        Ok(())
    }
}


#[derive(Debug, Clone, Default)]
pub enum PlayerAction {
    AddPlayer(Vec<ActionFor<AddPlayer>>),
    UpdateGamemode(Vec<ActionFor<VarInt>>),
    UpdateLatency(Vec<ActionFor<VarInt>>),
    UpdateDisplayName(Vec<ActionFor<TOption<String>>>),
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
    fn read(mut reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let action = VarInt::read_proto::<PV>(&mut reader)?;
        let ser_action = match action.0 {
            0 => PlayerAction::AddPlayer(Vec::read_proto::<PV>(reader)?),
            1 => PlayerAction::UpdateGamemode(Vec::read_proto::<PV>(reader)?),
            2 => PlayerAction::UpdateLatency(Vec::read_proto::<PV>(reader)?),
            3 => PlayerAction::UpdateDisplayName(Vec::read_proto::<PV>(reader)?),
            4 => PlayerAction::RemovePlayer,
            _ => PlayerAction::Unknown
        };
        Ok(Self(ser_action))
    }
}

impl<const PV: usize> MooshroomWritable<PV> for PlayerInfo {
    fn write(&self, mut writer: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        match &self.0 {
            PlayerAction::AddPlayer(p) => {
                VarInt(0).write_proto::<PV>(&mut writer)?;
                p.write_proto::<PV>(writer)?;
            },
            PlayerAction::UpdateGamemode(p) => {
                VarInt(1).write_proto::<PV>(&mut writer)?;
                p.write_proto::<PV>(writer)?;
            },
            PlayerAction::UpdateLatency(p) => {
                VarInt(2).write_proto::<PV>(&mut writer)?;
                p.write_proto::<PV>(writer)?;
            },
            PlayerAction::UpdateDisplayName(p) => { 
                VarInt(3).write_proto::<PV>(&mut writer)?;
                p.write_proto::<PV>(writer)?;
            },
            PlayerAction::RemovePlayer => {
                VarInt(4).write_proto::<PV>(&mut writer)?;
            },
            _ => {

            }
        }
        Ok(())
    }
}



#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x62)]
pub struct SystemChatMessage {
    pub json: String,
    pub is_overlay: bool
}