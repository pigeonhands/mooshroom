use std::{net::TcpStream, sync::mpsc};

use bevy::prelude::*;
use mooshroom::{proto::connection::MooshroomConnection, server::play::PlayStage};

use super::{
    entity::{self},
    player,
    population,
};

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
            _ => {
                //continue;
            }
        };
        if let Err(e) = tx.send(packet) {
            eprintln!("{:?}", e);
            return Err(mooshroom::prelude::MooshroomError::Other(e.into()));
        }
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
    mut query: Query<&mut entity::MobEntity, With<player::Player>>,
    mut ev_spawn_entity: EventWriter<entity::SpawnEntityEvent>,
    mut ev_entity: EventWriter<entity::UpdateEntityEvent>,
    mut ev_player: EventWriter<player::UpdatePlayerEvent>,
    mut ev_population: EventWriter<population::UpdatePopulationEvent>,
) {
    if let Some(rx) = &mc_con.rec {
        while let Ok(p) = rx.try_recv() {
            match p {
                PlayStage::SetHealth(p) => {
                    println!("{:#?}", p);
                    let mut player = query.single_mut();
                    player.health = p.health;
                }
                PlayStage::ChunkData(_) => {}
                PlayStage::SpawnEntity(c) => {
                    ev_spawn_entity.send(entity::SpawnEntityEvent(c));
                }
                PlayStage::UpdateEntityPosition(c) => {
                    ev_entity.send(entity::UpdateEntityEvent::UpdatePosition(c));
                }
                PlayStage::UpdateEntityPositionAndRotation(c) => {
                    ev_entity.send(entity::UpdateEntityEvent::UpdatePositionAndRotation(c));
                }
                PlayStage::RemoveEntities(c) => {
                    ev_entity.send(entity::UpdateEntityEvent::Remove(c));
                }
                PlayStage::TeleportEntity(c) => {
                    ev_entity.send(entity::UpdateEntityEvent::Teleport(c));
                }
                PlayStage::SynchronizePlayerPosition(c) => {
                    ev_player.send(player::UpdatePlayerEvent::SyncPosition(c));
                }
                PlayStage::SpawnPlayer(c) => {
                    ev_population.send(population::UpdatePopulationEvent::SpawnPlayer(c));
                }
                PlayStage::PlayerInfo(info) => {
                    ev_population.send(population::UpdatePopulationEvent::PlayerInfo(info));
                }
                _ => {}
            }
        }
    }
}
