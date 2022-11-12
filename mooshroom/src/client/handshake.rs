use mooshroom_core::{ varint::VarInt};
use mooshroom_macros::Mooshroom;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Mooshroom)]
#[repr(i32)]
#[value_type(VarInt)]
pub enum HandshakeState {
    #[default]
    Status = 1,
    Login = 2,
}

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x00)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

#[cfg(test)]
mod tests {
    use mooshroom_core::io::DEFAULT_PROTOCAL_VERSION;

    use super::*;

    fn check_packet_number<R, T: mooshroom_core::io::MooshroomPacket<DEFAULT_PROTOCAL_VERSION>>(
        _: T,
        id: i32,
    ) {
        assert_eq!(T::PACKET_ID, id);
    }

    #[test]
    fn test_derive_handshake() {
        check_packet_number::<Vec<i8>, _>(Handshake::default(), 0);
    }
}
