use mooshroom_core::{varint::VarInt, primitives::Identifier, io::{MooshroomReadProto, MooshroomReadable, MooshroomWritable, MooshroomWriteProto}};
use mooshroom_macros::Mooshroom;
use crate::core::{
    error::Result,
};

use super::npt;
use crate::containers::TOption;


#[derive(Debug, Clone, Default, Mooshroom)]
pub struct SlotData {
    pub item_id: VarInt,
    pub item_count: u8,
    pub nbt: npt::NptCompound,
}

pub type Slot = TOption<SlotData>;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x11)]
pub struct SetContainerContent {
    pub window_id: u8,
    pub state_id: VarInt,
    pub slot_data: Vec<Slot>,
    pub carried_item: Slot,
}


