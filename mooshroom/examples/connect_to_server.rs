use std::net::TcpStream;

use mooshroom::{
    proto::connection::MooshroomConnection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to 127.0.0.1:25565");

    let mut c = {
        let conn = TcpStream::connect("127.0.0.1:25565")?;
        MooshroomConnection::new(conn)
    };

    println!("Connected! handshaking...");

    c.handshake_offline()?;

    println!("reading play packets...");

    loop {
        let resp = c.next_play_packet()?;
        println!("{:?}", resp);
    }
}
