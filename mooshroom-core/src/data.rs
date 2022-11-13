use std::io;

use crate::{error::Result, io::{Protocal, MooshroomWritable, MooshroomReadable}, varint::VarInt};
pub trait MooshroomIdentifiable: Sized {
    type Type;
    fn from_id(id: Self::Type) -> Result<Self>;
    fn to_id(&self) -> Result<Self::Type>;
}

pub trait MooshroomCollection<const PV: Protocal>: Sized {
    fn read_one_of(id: VarInt, reader: &mut impl io::Read) -> Result<Self>;
    fn variant_id(&self) -> VarInt;
}

pub trait MooshroomUpdatable {
    type Type;
    fn update(&mut self, value: Self::Type);
    fn update_many(&mut self, value: impl IntoIterator<Item = Self::Type>) {
        for v in value {
            self.update(v);
        }
    }
}

pub trait MooshroomBitFlag:Sized {
    type Type;
    fn from_value(t: Self::Type)-> Self;
    fn to_value(&self) -> Self::Type;
}
impl<const PV:usize, T> MooshroomReadable<PV> for T
where T: MooshroomBitFlag,
T::Type: MooshroomReadable<PV> {
    fn read(reader: &mut impl io::Read) -> Result<Self> {
        let value = T::Type::read(reader)?;
        Ok(T::from_value(value))
    }
}

impl<const PV:usize, T> MooshroomWritable<PV> for T
where T: MooshroomBitFlag,
T::Type: MooshroomWritable<PV> {
    fn write(&self, writer: &mut impl io::Write) -> Result<()> {
        let value = self.to_value();
        value.write(writer)?;
        Ok(())
    }
}