use bevy::prelude::*;
use mooshroom::server::play::player;

use super::{entity};
use crate::camera::FlyCamera;

pub struct MinecraftPlayerPlugin;

impl Plugin for MinecraftPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdatePlayerEvent>();
        app.add_systems(Startup, add_player);
        app.add_systems(Update, handle_update_entity_event);
    }
}

#[derive(Event, Debug)]
pub enum UpdatePlayerEvent {
    SyncPosition(player::SynchronizePlayerPosition),
}

pub fn handle_update_entity_event(
    mut _commands: Commands,
    mut ev_entity: EventReader<UpdatePlayerEvent>,
    mut player: Query<(&mut Transform, Entity), With<Player>>,
) {
    let (mut transform, _) = player.single_mut();

    for ev in ev_entity.read() {
        match ev {
            UpdatePlayerEvent::SyncPosition(pos) => {
                println!("############### updated pos");
                let player::SynchronizePlayerPosition { x, y, z, yaw, pitch, .. } = pos;

                    let translation = &mut transform.translation;
                    translation.x = *x as f32;
                    translation.y = *y as f32;
                    translation.z = *z as f32;
                    transform.rotation =
                        Quat::from_euler(EulerRot::YXZ, yaw.to_radians(), pitch.to_radians(), 0.);
            }
            _ => {}
        }
    }
}

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
        .spawn((PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 3.0, 1.0))),
                material: materials.add(Color::TOMATO.into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            }, Player,
            entity::MobEntity::default(),
            FlyCamera{
                sensitivity: 10.,
                ..default()
            }
        ))
        .with_children(|c| {
            let camera_pos = Vec3::new(0., 5., 0.);
            let camera_transform =
                Transform::from_translation(camera_pos);//.looking_at(Vec3::ZERO, Vec3::Y);

            c.spawn(Camera3dBundle {
                transform: camera_transform,
                ..Default::default()
            });
        });
}
