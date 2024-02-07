use bevy::{prelude::*, utils::{HashMap, Uuid}};
use bevy_text_mesh::{TextMesh, TextMeshBundle, TextMeshFont};
use mooshroom::{core::primitives, server::play::population::{self, ActionFor, AddPlayer, PlayerAction, PlayerInfo}};

use super::entity::MCEntityId;

pub struct MinecraftPopulationPlugin;

impl Plugin for MinecraftPopulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdatePopulationEvent>();
        app.insert_resource(PopulationInfo::default());
        app.add_systems(Update, handle_update_population_event);
    }
}

#[derive(Debug, Default)]
pub struct PlayerState {
    name: String
}

#[derive(Resource, Debug, Default)]
pub struct PopulationInfo {
    players: HashMap<Uuid, PlayerState>
}

#[derive(Event, Debug)]
pub enum UpdatePopulationEvent {
    SpawnPlayer(population::SpawnPlayer),
    PlayerInfo(population::PlayerInfo)
}
pub fn handle_update_population_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut population: ResMut<PopulationInfo>,
    mut ev_entity: EventReader<UpdatePopulationEvent>,
) {

    for ev in ev_entity.read() {
        match ev {
            UpdatePopulationEvent::SpawnPlayer(player) => {
                let player_state = match population.players.get(&player.player_uuid) {
                    Some(p) => p,
                    None => {
                        println!("Tried spawining player that does not exist");
                        return
                    }
                };

                let primitives::Vec3 { x, y, z } = player.position;
                commands
                    .spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                            material: materials.add(Color::CRIMSON.into()),
                            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                            ..Default::default()
                        },
                        MCEntityId(player.entity_id),
                    ))
                    .with_children(|parent| {
                        let font: Handle<TextMeshFont> =
                            asset_server.load("fonts/FiraSans-Medium.ttf#mesh");

                        parent.spawn(TextMeshBundle {
                            text_mesh: TextMesh::new_with_color(
                                &player_state.name,
                                font,
                                Color::rgb(1., 1., 0.),
                            ),
                            transform: Transform::from_xyz(-1., 1.75, 0.),
                            ..Default::default()
                        });
                    });
            }
            UpdatePopulationEvent::PlayerInfo(PlayerInfo(action)) => {
                match action {
                    PlayerAction::AddPlayer(p) => {
                        for players in p {
                            let ActionFor {
                                uuid,
                                action: AddPlayer {
                                    name, ..
                                }
                            } = players;

                            let data = PlayerState {
                                name: name.into()
                            };

                            population.players.insert(*uuid, data);
                        } 
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
