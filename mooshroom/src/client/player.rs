use mooshroom_core::{io::{MooshroomWritable, MooshroomPacket, MooshroomReadable, MooshroomReadProto, MooshroomWriteProto}, varint::VarInt};
use mooshroom_core::error::MoshroomError;
#[derive(Debug, Copy, Clone, Default)]
pub enum Action {
    #[default]
    Respawn = 0,
    RequestStatus = 1
}
impl<const PV:usize> MooshroomPacket<PV> for Action {
    const PACKET_ID: VarInt = VarInt(0x07);
}
impl<const PV:usize> MooshroomReadable<PV> for Action {
    fn read(reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let p = match VarInt::read_proto::<PV>(reader)?.0 {
            0 => Self::Respawn,
            1 => Self::RequestStatus,
            i => return Err(MoshroomError::InvalidEnumVariant(i))
        };
        Ok(p)
    }
}
impl<const PV:usize> MooshroomWritable<PV> for Action {
    fn write(&self, writer: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        VarInt(*self as i32).write_proto::<PV>(writer)
    }
}
