use std::io;

use super::varint::VarInt;
use crate::error::Result;

pub const DEFAULT_PROTOCAL_VERSION: usize = 760;
pub type Protocal = usize;

pub trait MooshroomReadable<const PV: Protocal>: Sized {
    fn read(reader: &mut impl io::Read) -> Result<Self>;
}
pub trait MooshroomWritable<const PV: Protocal> {
    fn write(&self, writer: &mut impl io::Write) -> Result<()>;
}

pub trait MooshroomPacket<const PV: Protocal>:
    MooshroomReadable<PV> + MooshroomWritable<PV>
{
    const PACKET_ID: VarInt;
}

pub trait MooshroomCommand<const PV: Protocal>: MooshroomPacket<PV> {
    type Response: MooshroomPacket<PV>;
}

pub trait MooshroomCollection<const PV: Protocal>: Sized {
    fn read_one_of(id: VarInt, reader: &mut impl io::Read) -> Result<Self>;
}

pub trait MooshroomReadProto: Sized {
    fn read_proto<const PV: usize>(reader: &mut impl io::Read) -> Result<Self>
    where
        Self: MooshroomReadable<PV>;
}

impl<T> MooshroomReadProto for T {
    fn read_proto<const PV: usize>(reader: &mut impl io::Read) -> Result<Self>
    where
        Self: MooshroomReadable<PV>,
    {
        <Self as MooshroomReadable<PV>>::read(reader)
    }
}

pub trait MooshroomWriteProto {
    fn write_proto<const PV: usize>(&self, writer: &mut impl io::Write) -> Result<()>
    where
        Self: MooshroomWritable<PV>;
}

impl<T> MooshroomWriteProto for T {
    fn write_proto<const PV: usize>(&self, writer: &mut impl io::Write) -> Result<()>
    where
        Self: MooshroomWritable<PV>,
    {
        <Self as MooshroomWritable<PV>>::write(self, writer)
    }
}

pub trait MooshroomIdentifiable:Sized {
    type Type;
    fn from_id(id: Self::Type) -> Result<Self>;
    fn to_id(&self) -> Result<Self::Type>;
}