use std::io;

use super::varint::VarInt;
use crate::{data::MooshroomCollection, error::Result};

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

pub trait MooshroomReadProto: Sized {
    fn read_proto<const PV: usize>(reader: &mut impl io::Read) -> Result<Self>
    where
        Self: MooshroomReadable<PV>;
}

impl<T> MooshroomReadProto for T {
    #[inline]
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
    #[inline]
    fn write_proto<const PV: usize>(&self, writer: &mut impl io::Write) -> Result<()>
    where
        Self: MooshroomWritable<PV>,
    {
        <Self as MooshroomWritable<PV>>::write(self, writer)
    }
}

pub trait MooshroomCollectionProto {
    fn read_one_of_proto<const PV: usize>(id: VarInt, reader: &mut impl io::Read) -> Result<Self>
    where
        Self: MooshroomCollection<PV>;
    fn write_one_of_proto<const PV: usize>(&self, writer: &mut impl io::Write) -> Result<()>
    where
        Self: MooshroomCollection<PV>;

    fn variant_id_proto<const PV: usize>(&self) -> VarInt
    where
        Self: MooshroomCollection<PV>;
}

impl<T> MooshroomCollectionProto for T {
    #[inline]
    fn read_one_of_proto<const PV: usize>(id: VarInt, reader: &mut impl io::Read) -> Result<Self>
    where
        Self: MooshroomCollection<PV>,
    {
        <Self as MooshroomCollection<PV>>::read_one_of(id, reader)
    }
    #[inline]
    fn write_one_of_proto<const PV: usize>(&self, writer: &mut impl io::Write) -> Result<()>
    where
        Self: MooshroomCollection<PV>,
    {
        <Self as MooshroomCollection<PV>>::write_one_of(self, writer)
    }
    #[inline]
    fn variant_id_proto<const PV: usize>(&self) -> VarInt
    where
        Self: MooshroomCollection<PV>,
    {
        <Self as MooshroomCollection<PV>>::variant_id(&self)
    }
}
