mod chest;

use bevy::prelude::*;
use bevy_text_mesh::prelude::*;
use mooshroom::{
    core::{data::MooshroomIdentifiable, primitives, varint::VarInt},
    data::entity_data::EntityType,
    server::play::population,
};

#[derive(Component, Default)]
pub struct MobEntity {
    pub health: f32,
}

#[derive(Component, Default)]
pub struct MCEntityId(pub VarInt);

#[derive(Event)]
pub struct SpawnEntityEvent(pub population::SpawnEntity);

#[derive(Event, Debug)]
pub enum UpdateEntityEvent {
    UpdatePosition(population::UpdateEntityPosition),
    UpdatePositionAndRotation(population::UpdateEntityPositionAndRotation),
    Remove(population::RemoveEntities),
    Teleport(population::TeleportEntity),
}

pub struct MinecraftEntityPlugin;

impl Plugin for MinecraftEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateEntityEvent>()
            .add_event::<SpawnEntityEvent>()
            .add_systems(
                Update,
                (handle_entity_spawn_event, handle_update_entity_event),
            );
    }
}

fn get_model(e: EntityType, asset_server: &Res<AssetServer>) -> Option<Handle<Scene>> {
    match e {
        EntityType::Bat => Some(asset_server.load("models/bat.gltf#Scene0")),
        EntityType::Cow => Some(asset_server.load("models/cow.gltf#Scene0")),
        EntityType::Zombie => Some(asset_server.load("models/zombie.gltf#Scene0")),
        EntityType::Sheep => Some(asset_server.load("models/sheep.gltf#Scene0")),
        EntityType::Creeper => Some(asset_server.load("models/creeper.gltf#Scene0")),
        EntityType::Skeleton => Some(asset_server.load("models/skeleton.gltf#Scene0")),
        EntityType::GlowSquid => Some(asset_server.load("models/glow_squid.gltf#Scene0")),
        EntityType::Pig => Some(asset_server.load("models/pig.gltf#Scene0")),
        EntityType::Chicken => Some(asset_server.load("models/chicken.gltf#Scene0")),
        EntityType::Enderman => Some(asset_server.load("models/enderman.gltf#Scene0")),
        EntityType::Spider => Some(asset_server.load("models/spider.gltf#Scene0")),
        _ => None,
    }
}

pub fn handle_entity_spawn_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut ev_entity: EventReader<SpawnEntityEvent>,
) {
    for ev in ev_entity.read() {
        let entity = &ev.0;
        let primitives::Vec3 { x, y, z } = entity.position;

        if let Some(model) = get_model(entity.entity_type, &asset_server) {
            commands
                .spawn(SceneBundle {
                    scene: model,
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..Default::default()
                })
                .insert(MCEntityId(entity.entity_id))
                .with_children(|parent| {
                    let font: Handle<TextMeshFont> =
                        asset_server.load("fonts/FiraSans-Medium.ttf#mesh");
                    parent.spawn(TextMeshBundle {
                        text_mesh: TextMesh::new_with_color(
                            entity.entity_type.to_id().unwrap(),
                            font,
                            Color::rgb(1., 1., 0.),
                        ),
                        transform: Transform::from_xyz(-1., 1.75, 0.),
                        ..Default::default()
                    });
                });
        } else {
            commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                        material: materials.add(Color::rgb(20.0, 10.0, 50.0).into()),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..Default::default()
                    },
                    MCEntityId(entity.entity_id),
                ))
                .with_children(|parent| {
                    let font: Handle<TextMeshFont> =
                        asset_server.load("fonts/FiraSans-Medium.ttf#mesh");

                    parent.spawn(TextMeshBundle {
                        text_mesh: TextMesh::new_with_color(
                            entity.entity_type.to_id().unwrap(),
                            font,
                            Color::rgb(1., 1., 0.),
                        ),
                        transform: Transform::from_xyz(-1., 1.75, 0.),
                        ..Default::default()
                    });
                });
        }
    }
}

pub fn handle_update_entity_event(
    mut commands: Commands,
    mut ev_entity: EventReader<UpdateEntityEvent>,
    mut entities: Query<(&MCEntityId, &mut Transform, Entity)>,
) {
    for ev in ev_entity.read() {
        match ev {
            UpdateEntityEvent::UpdatePosition(e) => {
                for (_, mut transform, _) in entities
                    .iter_mut()
                    .filter(|(MCEntityId(ent_id), _, _)| *ent_id == e.entity_id)
                {
                    let factor = 128.0 * 32.0;
                    let translation = &mut transform.translation;
                    translation.x += (e.delta.x as f32) / factor;
                    translation.y += (e.delta.y as f32) / factor;
                    translation.z += (e.delta.z as f32) / factor;
                    //  println!("updating entity {:?}", e);
                }
            }
            UpdateEntityEvent::UpdatePositionAndRotation(e) => {
                for (_, mut transform, _) in entities
                    .iter_mut()
                    .filter(|(ent_id, _, _)| ent_id.0 == e.entity_id)
                {
                    let factor = 128.0 * 32.0;

                    let translation = &mut transform.translation;
                    translation.x += (e.delta.x as f32) / factor;
                    translation.y += (e.delta.y as f32) / factor;
                    translation.z += (e.delta.z as f32) / factor;

                    transform.rotation =
                        Quat::from_euler(EulerRot::YXZ, e.yaw.to_radians(), e.pitch.to_radians(), 0.);
                    //   println!("updating entity {:?}", e);
                }
            }
            UpdateEntityEvent::Remove(e) => {
                for (_, _, entity) in entities
                    .iter_mut()
                    .filter(|(ent_id, _, _)| e.entities.contains(&ent_id.0))
                {
                    commands.get_entity(entity).unwrap().despawn_recursive();
                }
            }
            UpdateEntityEvent::Teleport(e) => {
                for (_, mut transform, _) in entities
                    .iter_mut()
                    .filter(|(ent_id, _, _)| ent_id.0 == e.entity_id)
                {
                    let translation = &mut transform.translation;
                    translation.x = e.location.x as f32;
                    translation.y = e.location.y as f32;
                    translation.z = e.location.z as f32;
                    transform.rotation =
                        Quat::from_euler(EulerRot::YXZ, e.yaw.to_radians(), e.pitch.to_radians(), 0.);
                }
            }
        }
    }
}
