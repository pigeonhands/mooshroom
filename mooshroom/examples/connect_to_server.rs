use std::net::TcpStream;

use mooshroom::{
    client::{
        handshake::{Handshake, HandshakeState},
        login::LoginStart,
        status::StatusRequest,
    },
    core::ProtocolVersion,
    proto::MooshroomProto,
    server::{
        play::PlayStage,
        login::{LoginStage, SetCompression},
    }
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to 127.0.0.1:25565");

    let mut m = {
        let conn = TcpStream::connect("127.0.0.1:25565")?;
        MooshroomProto::new(conn)
    };

    println!("Connected!");

    m.write_packet(&Handshake {
        server_address: "127.0.0.1".into(),
        server_port: 25565,
        protocol_version: (ProtocolVersion::V1_19_2 as i32).into(),
        next_state: HandshakeState::Login,
    })?;

    println!("Sending offline login");

    m.write_packet(&LoginStart {
        name: "mooshroom".into(),
        player_uuid: None.into(),
        sig_data: None.into(),
    })?;

    println!("Waiting for response");

    loop {
        let resp: LoginStage = m.read_one_of()?;
        println!("{:?}", resp);

        match resp {
            LoginStage::SetCompression(n) => m.codec.set_compression(n.threshold.0),
            LoginStage::Success(_) => break,
            _ => {}
        }
    }

    println!("going to play stage");

    loop {
        let resp: PlayStage = m.read_one_of()?;
        println!("{:?}", resp);

        match resp {
            
            _ => {}
        }
    }

    Ok(())
}
