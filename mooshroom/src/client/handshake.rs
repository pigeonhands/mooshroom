use mooshroom_core::{
    error::{MoshroomError, Result},
    io::{MooshroomReadable, MooshroomWritable},
    varint::VarInt,
};
use mooshroom_macros::Mooshroom;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HandshakeState {
    #[default]
    Status = 1,
    Login = 2,
}

impl From<HandshakeState> for VarInt {
    fn from(h: HandshakeState) -> Self {
        Self(h as i32)
    }
}
impl From<&HandshakeState> for VarInt {
    fn from(h: &HandshakeState) -> Self {
        Self(*h as i32)
    }
}

impl TryFrom<VarInt> for HandshakeState {
    type Error = MoshroomError;

    fn try_from(value: VarInt) -> std::result::Result<Self, Self::Error> {
        Ok(match value.0 {
            1 => Self::Status,
            2 => Self::Login,
            i => return Err(MoshroomError::InvalidHandshakeState(i)),
        })
    }
}

impl MooshroomReadable for HandshakeState {
    fn read(reader: impl std::io::Read, version: mooshroom_core::ProtocolVersion) -> Result<Self> {
        let val = VarInt::read(reader, version)?;
        val.try_into()
    }
}

impl MooshroomWritable for HandshakeState {
    fn write(
        &self,
        writer: impl std::io::Write,
        version: mooshroom_core::ProtocolVersion,
    ) -> Result<()> {
        let vi: VarInt = self.into();
        vi.write(writer, version)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_packet_number<R, T: mooshroom_core::io::MooshroomPacket>(_: T, id: i32) {
        assert_eq!(T::PACKET_ID, id);
    }

    #[test]
    fn test_derive_handshake() {
        check_packet_number::<Vec<i8>, _>(Handshake::default(), 1);
    }
}
