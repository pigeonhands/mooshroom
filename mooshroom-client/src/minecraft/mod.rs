mod net;
mod entity;
mod player;

use std::collections::HashMap;
use bevy::prelude::*;
use mooshroom::core::varint::VarInt;

#[derive(Default, Debug, Resource)]
pub struct NetEntities {
    pub entities: HashMap<VarInt, Entity>,
}

pub struct MinecraftPlugin;

impl Plugin for MinecraftPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(net::MinecraftConnection { rec: None });
        app.insert_resource(NetEntities::default());
        app.add_plugin(entity::MinecraftEntityPlugin);
        app.add_startup_system(net::connect_to_server);
        app.add_startup_system(player::add_player);
        app.add_system(net::handle_messages);
    }
}
