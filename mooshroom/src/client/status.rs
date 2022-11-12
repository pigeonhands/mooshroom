use mooshroom_macros::Mooshroom;

use crate::server::status::{PingResponse, StatusResponse};

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x0)]
#[response(StatusResponse)]
pub struct StatusRequest;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0x01)]
#[response(PingResponse)]
pub struct PingRequest(u64);

#[cfg(test)]
mod tests {
    use mooshroom_core::io::*;

    use super::*;

    fn check_packet_number<T: mooshroom_core::io::MooshroomPacket<DEFAULT_PROTOCAL_VERSION>>(
        _: T,
        id: i32,
    ) {
        assert_eq!(T::PACKET_ID, id);
    }

    #[test]
    fn test_derive_server_status() {
        check_packet_number(StatusRequest::default(), 0);
    }

    #[test]
    fn test_server_status_no_body() {
        let mut b = Vec::new();

        <StatusRequest as MooshroomWritable<DEFAULT_PROTOCAL_VERSION>>::write(
            &StatusRequest::default(),
            &mut b,
        )
        .unwrap();
        assert!(b.is_empty())
    }
}
