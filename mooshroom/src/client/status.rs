use mooshroom_macros::Mooshroom;

use crate::server::status::StatusResponse;

#[derive(Debug, Clone, Default, Mooshroom)]
#[packet_id(0)]
#[response(StatusResponse)]
pub struct StatusRequest;

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
