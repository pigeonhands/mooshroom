use mooshroom_macros::Mooshroom;
use crate::server::status::StatusResponse;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
#[response(StatusResponse)]
pub struct StatusRequest;


#[cfg(test)]
mod tests {
    use super::*;
    use mooshroom_core::io::*;

    fn check_packet_number<T: mooshroom_core::io::MooshroomPacket>(_: T, id: i32) {
        assert_eq!(T::PACKET_ID, id);
    }

    #[test]
    fn test_derive_server_status() {
        check_packet_number(StatusRequest::default(), 1);
    }

    #[test]
    fn test_server_status_no_body() {
        let mut b = Vec::new();
        StatusRequest::default().write(&mut b, mooshroom_core::ProtocolVersion::V1_16_5).unwrap();
        assert!(b.is_empty())
    }
}