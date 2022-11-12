use std::{net::TcpStream, sync::mpsc};

use bevy::prelude::*;
use log::{error, info};
use mooshroom::{proto::connection::MooshroomConnection, server::play::PlayStage};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use crate::player;

pub struct MinecraftConnection {
    rec: Option<mpsc::Receiver<PlayStage>>,
}
pub struct MinecraftPlugin;

impl Plugin for MinecraftPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(MinecraftConnection { rec: None });
        app.add_startup_system(connect_to_server);
        app.add_startup_system(add_player);
        app.add_system(handle_messages);
    }
}

fn run_server(tx: mpsc::Sender<PlayStage>) -> mooshroom::core::error::Result<()> {
    info!("Connecting to 127.0.0.1:25565");

    let mut c = {
        let conn = TcpStream::connect("127.0.0.1:25565")?;
        MooshroomConnection::new(conn)
    };

    info!("Connected! handshaking...");

    c.handshake_offline()?;

    info!("reading play packets...");

    loop {
        let packet = c.next_play_packet()?;
        //info!("{:?}", packet);
        match &packet {
            PlayStage::KeepAlive(id) => {
                println!("Sent keepalive");
                c.respond_to_keep_alive(id.0)?;
            }
            PlayStage::SetHealth(p) => {
                println!("{:#?}", p);
                if p.health <= 0. {
                    println!("respawning...");
                    c.respawn()?;
                }
            }
            PlayStage::PlayerChatMessage(c) => {
                info!("{}", c.plain_message)
            }
            PlayStage::Respawn(p) => {
                info!("{:#?}", p);
            }
            PlayStage::CombatDeath(p) => {
                info!("{:#?}", p);
            }
            _ => {
                //continue;
            }
        };
        tx.send(packet).unwrap();
    }
}

fn connect_to_server(mut mc_con: NonSendMut<MinecraftConnection>) {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        if let Err(e) = run_server(tx) {
            error!("connection ended!. {}", e);
        }
    });
    mc_con.rec = Some(rx);
}

fn handle_messages(
    mc_con: NonSendMut<MinecraftConnection>,
    mut query: Query<&mut player::EntityHealth, With<player::Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(rx) = &mc_con.rec {
        while let Ok(p) = rx.try_recv() {
            match p {
                PlayStage::SetHealth(p) => {
                    println!("{:#?}", p);
                    let mut player = query.single_mut();
                    player.health = p.health;
                }
                PlayStage::ChunkData(c) => {
                    if c.entity_blocks.len() > 0 {
                        info!("{:?}", c.entity_blocks);
                    }
                    for b in c.entity_blocks {
                        let (x, z) = b.xz.unpack();
                        commands.spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box::new(3.0, 0.1, 1.0))),
                            material: materials.add(Color::rgb(20.0, 10.0, 50.0).into()),
                            transform: Transform::from_xyz(x as f32, b.y as f32, z as f32),
                            ..Default::default()
                        });
                    }
                },
                _ => {}
            }
        }
    }
}

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(3.0, 0.1, 1.0))),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 30.5, 0.0),
            ..Default::default()
        }).with_children(|c| {
            let camera_pos = Vec3::new(-2.0, 2.5, 1.0);
            let camera_transform = Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y);
        
            c
            .spawn_bundle(Camera3dBundle {
                transform: camera_transform,
                ..Default::default()
            })
            .insert_bundle(OrbitCameraBundle::new(
                OrbitCameraController::default(),
                Vec3::new(-2.0, 5.0, 5.0),
                Vec3::new(0., 0., 0.),
            ));
        })
        .insert(player::Player)
        .insert(player::EntityHealth::default());
}
