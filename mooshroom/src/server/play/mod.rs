pub mod npt;

use mooshroom_macros::{MooshroomCollection, Mooshroom};


pub enum GameMode {
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
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub dimention_names: Vec<String>,
    pub npt: npt::NptCompound,
}


#[derive(Debug, Clone, MooshroomCollection)]
pub enum PlayStage {
    Login(LoginPlay)
}
