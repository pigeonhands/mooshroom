use std::mem::MaybeUninit;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    error::MooshroomError,
    io::{MooshroomReadProto, MooshroomReadable, MooshroomWritable, MooshroomWriteProto},
    varint::VarInt,
};

pub type Identifier = String;

macro_rules! impl_rw_primitive {
    ($e:ident) => {
            paste::expr! {
            impl<const PV : usize> MooshroomReadable<PV> for $e {
                fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
                    reader.[<  read_ $e >]::<BigEndian>().map_err(MooshroomError::IoError)
                }
            }

            impl<const PV : usize> MooshroomWritable<PV> for $e {
                fn write(
                    &self,
                    writer: &mut impl std::io::Write
                ) -> crate::error::Result<()> {
                    writer.[< write_ $e >]::<BigEndian>(*self).map_err(MooshroomError::IoError)?;
                    Ok(())
                }
            }
        }
    };
    ($e:ident, $($es:ident),*) => {
        impl_rw_primitive! { $e }
        impl_rw_primitive! { $($es),+ }
    };
}

impl_rw_primitive!(u16, i16, i32, u32, u64, i64, u128, i128, f32, f64);

impl<const PV: usize> MooshroomReadable<PV> for bool {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        reader
            .read_i8()
            .map(|i| i != 0)
            .map_err(MooshroomError::IoError)
    }
}

impl<const PV: usize> MooshroomWritable<PV> for bool {
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        writer
            .write_u8(*self as u8)
            .map_err(MooshroomError::IoError)
    }
}

impl<const PV: usize> MooshroomReadable<PV> for i8 {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        reader.read_i8().map_err(MooshroomError::IoError)
    }
}

impl<const PV: usize> MooshroomWritable<PV> for i8 {
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        writer.write_i8(*self).map_err(MooshroomError::IoError)
    }
}

impl<const PV: usize> MooshroomReadable<PV> for u8 {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        reader.read_u8().map_err(MooshroomError::IoError)
    }
}

impl<const PV: usize> MooshroomWritable<PV> for u8 {
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        writer.write_u8(*self).map_err(MooshroomError::IoError)
    }
}

impl<const PV: usize> MooshroomReadable<PV> for String {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        let len = <VarInt as MooshroomReadable<PV>>::read(reader)?;

        let s = {
            let mut buffer: Vec<u8> = vec![0; len.0 as usize];
            reader.read_exact(&mut buffer)?;
            String::from_utf8(buffer).map_err(MooshroomError::InvalidString)?
        };
        Ok(s)
    }
}

impl<const PV: usize> MooshroomWritable<PV> for String {
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        <VarInt as MooshroomWritable<PV>>::write(&VarInt(self.len() as i32), writer)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<const PV: usize, T> MooshroomReadable<PV> for Vec<T>
where
    T: MooshroomReadable<PV>,
{
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        let len = <VarInt as MooshroomReadable<PV>>::read(reader)?.0 as usize;

        let mut buffer = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(T::read(reader)?);
        }
        Ok(buffer)
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for Vec<T>
where
    T: MooshroomWritable<PV>,
{
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        <VarInt as MooshroomWritable<PV>>::write(&VarInt(self.len() as i32), writer)?;
        for p in self.iter() {
            p.write(writer)?;
        }
        Ok(())
    }
}

impl<const PV: usize, T, const N: usize> MooshroomReadable<PV> for [T; N]
where
    T: MooshroomReadable<PV> + Sized,
{
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        unsafe {
            let mut buffer: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
            for i in buffer.iter_mut() {
                i.write(T::read(reader)?);
            }
            Ok(buffer.as_ptr().cast::<[T; N]>().read())
        }
    }
}

impl<const PV: usize, T, const N: usize> MooshroomWritable<PV> for [T; N]
where
    T: MooshroomWritable<PV>,
{
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        for i in self.iter() {
            i.write(writer)?;
        }
        Ok(())
    }
}

#[cfg(feature = "uuid")]
impl<const PV: usize> MooshroomReadable<PV> for uuid::Uuid {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        let b = <[u8; 16] as MooshroomReadable<PV>>::read(reader)?;
        Ok(uuid::Uuid::from_bytes(b))
    }
}

#[cfg(feature = "uuid")]
impl<const PV: usize> MooshroomWritable<PV> for uuid::Uuid {
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        let s = self.as_bytes();
        <[u8; 16] as MooshroomWritable<PV>>::write(s, writer)
    }
}

const I26_MASK: i32 = 0b1111111111_1111111111_111111;
const I12_MASK: i16 = 0b1111111111_11;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32, //i26
    pub z: i32, //i26
    pub y: i16, //i12
}

impl<const PV: usize> MooshroomReadable<PV> for Position {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        let base = u64::read_proto::<PV>(reader)?;
        Ok(Self {
            x: (base >> 38) as i32 & I26_MASK,
            z: (base >> 12) as i32 & I26_MASK,
            y: base as i16 & I12_MASK,
        })
    }
}

impl<const PV: usize> MooshroomWritable<PV> for Position {
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        let mut buffer = [0; 8];
        buffer[..26].copy_from_slice(&(self.x & I26_MASK).to_be_bytes()[6..]);
        buffer[26..26 + 26].copy_from_slice(&(self.z & I26_MASK).to_be_bytes()[6..]);
        buffer[26 + 26..].copy_from_slice(&(self.y & I12_MASK).to_be_bytes()[4..]);
        writer.write_all(&buffer)?;
        Ok(())
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<const PV: usize, T> MooshroomWritable<PV> for Vec3<T>
where
    T: MooshroomWritable<PV>,
{
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        self.x.write_proto::<PV>(writer)?;
        self.y.write_proto::<PV>(writer)?;
        self.z.write_proto::<PV>(writer)
    }
}

impl<const PV: usize, T> MooshroomReadable<PV> for Vec3<T>
where
    T: MooshroomReadable<PV>,
{
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        Ok(Self {
            x: T::read_proto::<PV>(reader)?,
            y: T::read_proto::<PV>(reader)?,
            z: T::read_proto::<PV>(reader)?,
        })
    }
}

impl<const PV: usize, T> MooshroomReadable<PV> for Option<T>
where
    T: MooshroomReadable<PV>,
{
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        if bool::read_proto::<PV>(reader)? {
            Ok(Some(T::read(reader)?))
        } else {
            Ok(None)
        }
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for Option<T>
where
    T: MooshroomWritable<PV>,
{
    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
        if let Some(t) = &self {
            true.write_proto::<PV>(writer)?;
            t.write(writer)?;
        } else {
            false.write_proto::<PV>(writer)?;
        }
        Ok(())
    }
}


impl<const PV: usize> MooshroomReadable<PV> for (){
    fn read(_: &mut impl std::io::Read) -> crate::error::Result<Self> {
        Ok(())
    }
}

impl<const PV: usize> MooshroomWritable<PV> for () {
    fn write(&self, _: &mut impl std::io::Write) -> crate::error::Result<()> {
        Ok(())
    }
}
