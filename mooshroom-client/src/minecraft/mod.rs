mod entity;
mod net;
mod player;
mod population;

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
        app.add_plugins((
            entity::MinecraftEntityPlugin,
            player::MinecraftPlayerPlugin,
            population::MinecraftPopulationPlugin,
        ));
        app.add_systems(Startup, (net::connect_to_server,));
        app.add_systems(Update, net::handle_messages);
    }
}
