use mooshroom_core::varint::VarInt;
use mooshroom_macros::Mooshroom;

use super::nbt;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x10)]
pub struct CloseContainer(pub u8);

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct SlotData {
    pub item_id: VarInt,
    pub item_count: u8,
    pub nbt: nbt::NptCompound,
}

pub type Slot = Option<SlotData>;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x11)]
pub struct SetContainerContent {
    pub window_id: u8,
    pub state_id: VarInt,
    pub slot_data: Vec<Slot>,
    pub carried_item: Slot,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x13)]
pub struct SetContainerSlot {
    pub window_id: u8,
    pub state_id: VarInt,
    pub slot: u16,
    pub carried_item: Slot,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x1D)]
pub struct OpenHorseScreen {
    //pub window_id: u8,
    //pub slot_count: VarInt,
    //pub entity_id: u32,
}
