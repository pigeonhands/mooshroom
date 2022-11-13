mod net;
mod entity;
mod player;


use bevy::prelude::*;

pub struct MinecraftPlugin;

impl Plugin for MinecraftPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(net::MinecraftConnection { rec: None });
        app.add_startup_system(net::connect_to_server);
        app.add_startup_system(player::add_player);
        app.add_system(net::handle_messages);
    }
}
