use std::net::TcpStream;

use super::MooshroomProto;
use crate::{
    client::{
        handshake::{Handshake, HandshakeState},
        login::LoginStart, metadata::KeepAliveResponse,
        player
    },
    core::error::Result,
    server::{
        login::{LoginStage, LoginSuccess},
        play::PlayStage,
    },
};

pub enum Stage {
    Handshake,
    Login,
    Play(uuid::Uuid),
}

pub struct MooshroomConnection {
    sock: MooshroomProto<TcpStream>,
    stage: Stage,
}

impl MooshroomConnection {
    pub fn new(sock: TcpStream) -> Self {
        Self {
            sock: MooshroomProto::new(sock),
            stage: Stage::Handshake,
        }
    }

    pub fn stage(&self) -> &Stage {
        &self.stage
    }

    pub fn handshake_offline(&mut self) -> Result<()> {
        let ep = self.sock.inner.peer_addr()?;

        self.sock.write_packet(&Handshake {
            server_address: ep.ip().to_string(),
            server_port: ep.port(),
            protocol_version: self.sock.protocal_version().into(),
            next_state: HandshakeState::Login,
        })?;

        self.sock.write_packet(&LoginStart {
            name: "mooshroom".into(),
            player_uuid: None.into(),
            sig_data: None.into(),
        })?;

        loop {
            let resp: LoginStage = self.sock.read_one_of()?;

            match resp {
                LoginStage::SetCompression(n) => self.sock.codec.set_compression(n.threshold.0),
                LoginStage::Success(LoginSuccess { uuid, .. }) => {
                    self.stage = Stage::Play(uuid);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn next_play_packet(&mut self) -> Result<PlayStage> {
        self.sock.read_one_of()
    }

    pub fn respond_to_keep_alive(&mut self, id: i64) -> Result<()> {
       self.sock.write_packet(&KeepAliveResponse(id))
    }

    pub fn respawn(&mut self) -> Result<()> {
        self.sock.write_packet(&player::Action::Respawn)
    }
}
