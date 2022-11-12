use std::net::TcpStream;

use mooshroom::{proto::connection::MooshroomConnection, server::play::PlayStage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("Connecting to 127.0.0.1:25565");

    let mut c = {
        let conn = TcpStream::connect("127.0.0.1:25565")?;
        MooshroomConnection::new(conn)
    };

    println!("Connected! handshaking...");

    c.handshake_offline()?;

    println!("reading play packets...");

    loop {
        let packet = c.next_play_packet()?;

        match packet {
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
                println!("{}", c.plain_message)
            }
            PlayStage::Respawn(p) => {
                println!("{:#?}", p);
            }
            PlayStage::CombatDeath(p) => {
                println!("{:#?}", p);
            }
            _ => {}
        }
    }
}
