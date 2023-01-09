use std::io;

use crate::{
    error::Result,
    io::{MooshroomReadable, MooshroomWritable, Protocal},
    varint::VarInt,
};
pub trait MooshroomIdentifiable: Sized {
    type Type;
    fn from_id(id: Self::Type) -> Result<Self>;
    fn to_id(&self) -> Result<Self::Type>;
}

pub trait MooshroomValue: Sized {
    type Type;
    fn from_value(id: Self::Type) -> Result<Self>;
    fn to_value(&self) -> Result<Self::Type>;
}

pub trait MooshroomCollection<const PV: Protocal>: Sized {
    fn read_one_of(id: VarInt, reader: &mut impl io::Read) -> Result<Self>;
    fn write_one_of(&self, writer: &mut impl io::Write) -> Result<()>;
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

pub trait MooshroomBitFlag: Sized {
    type Type;
    fn from_value(t: Self::Type) -> Self;
    fn to_value(&self) -> Self::Type;
}
impl<const PV: usize, T> MooshroomReadable<PV> for T
where
    T: MooshroomBitFlag,
    T::Type: MooshroomReadable<PV>,
{
    fn read(reader: &mut impl io::Read) -> Result<Self> {
        let value = T::Type::read(reader)?;
        Ok(T::from_value(value))
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for T
where
    T: MooshroomBitFlag,
    T::Type: MooshroomWritable<PV>,
{
    fn write(&self, writer: &mut impl io::Write) -> Result<()> {
        let value = self.to_value();
        value.write(writer)?;
        Ok(())
    }
}

pub trait MooshroomToBitField<T: MooshroomBitFlag> {
    fn to_bitflag(mask: T::Type, value: Self) -> T::Type;
    fn from_bitflag(mask: T::Type, value: T::Type) -> Self;
}

impl<T> MooshroomToBitField<T> for bool
where
    T: MooshroomBitFlag<Type = u8>,
{
    #[inline]
    fn to_bitflag(mask: u8, value: Self) -> u8 {
        if value {
            mask
        } else {
            0
        }
    }
    #[inline]
    fn from_bitflag(mask: u8, value: u8) -> Self {
        (mask & value) == mask
    }
}

impl<T> MooshroomToBitField<T> for u8
where
    T: MooshroomBitFlag<Type = u8>,
{
    #[inline]
    fn to_bitflag(mask: u8, value: u8) -> u8 {
        (value << mask.trailing_zeros()) & mask
    }
    #[inline]
    fn from_bitflag(mask: u8, value: u8) -> u8 {
        (value & mask) >> mask.trailing_zeros()
    }
}
