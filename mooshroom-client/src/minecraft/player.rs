use bevy::prelude::*;
use super::{entity, player};
use crate::camera::{FlyCamera};

#[derive(Component)]
pub struct Player;

#[derive(Clone, Component, Copy, Debug)]
pub struct FpsCameraController {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub translate_sensitivity: f32,
    pub smoothing_weight: f32,
}

impl Default for FpsCameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(0.2),
            translate_sensitivity: 2.0,
            smoothing_weight: 0.9,
        }
    }
}



pub fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(3.0, 0.1, 1.0))),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 30.5, 0.0),
            ..Default::default()
        }).with_children(|c| {
            let camera_pos = Vec3::new(-2.0, 2.5, 1.0);
            let camera_transform = Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y);
        
            c
            .spawn(Camera3dBundle {
                transform: camera_transform,
                ..Default::default()
            }).insert( FlyCamera{
                sensitivity: 10.,
                ..FlyCamera::default()
            },
        );
        }).insert((
            player::Player,
            entity::Entity::default()
        ));
}
