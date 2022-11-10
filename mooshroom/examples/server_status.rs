use std::net::TcpStream;

use mooshroom::{
    client::{
        handshake::{Handshake, HandshakeState},
        status::StatusRequest,
    },
    proto::MooshroomProto,
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
        protocol_version: (-1).into(),
        next_state: HandshakeState::Status,
    })?;

    let status = m.send_command(&StatusRequest)?;
    println!("{:?}", status);

    Ok(())
}
