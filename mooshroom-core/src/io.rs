use std::io;

use super::{varint::VarInt, ProtocolVersion};
use crate::error::Result;

pub trait MooshroomReadable: Sized {
    fn read(reader: &mut impl io::Read, version: ProtocolVersion) -> Result<Self>;
}
pub trait MooshroomWritable {
    fn write(&self, writer: &mut impl io::Write, version: ProtocolVersion) -> Result<()>;
}

pub trait MooshroomPacket: MooshroomReadable + MooshroomWritable {
    const PACKET_ID: VarInt;
}

pub trait MooshroomCommand {
    type Response : MooshroomPacket;
}