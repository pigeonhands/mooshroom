use mooshroom_core::varint::VarInt;
use mooshroom_macros::Mooshroom;

use super::npt;
use crate::{
    containers::TOption,
    core::primitives::{Identifier, Position},
};

pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x0b)]
pub struct ChangeDifficulty {
    pub difficulty: u8,
    pub difficulty_locked: bool,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x1A)]
pub struct EntityEvent {
    pub entity_id: i32,
    pub status: u8,
}

pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug, Clone, Default, Mooshroom)]
pub struct DeathLocation {
    pub dimention_name: Identifier,
    pub location: Position,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x25)]
pub struct LoginPlay {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub dimention_names: Vec<String>,
    pub npt: npt::NptCompound,
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
    pub death_location: TOption<DeathLocation>,
}
