use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Copy, Clone, Default, Debug, Component)]
pub struct EntityHealth {
    pub health: f32,
}
