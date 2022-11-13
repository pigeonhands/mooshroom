use bevy::prelude::*;
use std::{net::TcpStream, sync::mpsc};
use mooshroom::{proto::connection::MooshroomConnection, server::play::PlayStage};

use super::{entity, player};

pub struct MinecraftConnection {
    pub rec: Option<mpsc::Receiver<PlayStage>>,
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
            },
            PlayStage::SpawnEntity(p) => {
                info!("{:#?}", p);
            },
            _ => {
                //continue;
            }
        };
        tx.send(packet).unwrap();
    }
}

pub fn connect_to_server(mut mc_con: NonSendMut<MinecraftConnection>) {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        if let Err(e) = run_server(tx) {
            error!("connection ended!. {}", e);
        }
    });
    mc_con.rec = Some(rx);
}

pub fn handle_messages(
    mc_con: NonSendMut<MinecraftConnection>,
    mut query: Query<&mut entity::Entity, With<player::Player>>,
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
                        commands.spawn(PbrBundle {
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
