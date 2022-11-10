use std::net::TcpStream;

use mooshroom::{
    client::{
        handshake::{Handshake, HandshakeState},
        login::LoginStart,
        status::StatusRequest,
    },
    core::ProtocolVersion,
    proto::MooshroomProto,
    server::login::LoginSuccess,
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
        protocol_version: (ProtocolVersion::V1_13_2 as i32).into(),
        next_state: HandshakeState::Login,
    })?;

    println!("Sending offline login");

    m.write_packet(&LoginStart {
        name: "mooshroom".into(),
        player_uuid: Some(uuid::Uuid::parse_str(
            "156062c2-952f-3cd3-b877-c03b38f69c30",
        )?)
        .into(),
        sig_data: None.into(),
    })?;

    println!("Waiting for response");

    let resp: LoginSuccess = m.read_packet()?;

    println!("{:?}", resp);

    Ok(())
}
