use std::mem::MaybeUninit;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    error::MoshroomError,
    io::{MooshroomReadable, MooshroomWritable},
    varint::VarInt,
};

macro_rules! impl_rw_primitive {
    ($e:ident) => {
            paste::expr! {
            impl MooshroomReadable for $e {
                fn read(mut reader: impl std::io::Read, _: crate::ProtocolVersion) -> crate::error::Result<Self> {
                    reader.[<  read_ $e >]::<BigEndian>().map_err(MoshroomError::IoError)
                }
            }

            impl MooshroomWritable for $e {
                fn write(
                    &self,
                    mut writer: impl std::io::Write,
                    _: crate::ProtocolVersion,
                ) -> crate::error::Result<()> {
                    writer.[< write_ $e >]::<BigEndian>(*self).map_err(MoshroomError::IoError)?;
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

impl_rw_primitive!(
    //u8,
    //i8,
    u16, i16, u64, i64, u128, i128
);

impl MooshroomReadable for bool {
    fn read(
        mut reader: impl std::io::Read,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<Self> {
        reader
            .read_i8()
            .map(|i| i != 0)
            .map_err(MoshroomError::IoError)
    }
}

impl MooshroomWritable for bool {
    fn write(
        &self,
        mut writer: impl std::io::Write,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        writer.write_u8(*self as u8).map_err(MoshroomError::IoError)
    }
}

impl MooshroomReadable for i8 {
    fn read(
        mut reader: impl std::io::Read,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<Self> {
        reader.read_i8().map_err(MoshroomError::IoError)
    }
}

impl MooshroomWritable for i8 {
    fn write(
        &self,
        mut writer: impl std::io::Write,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        writer.write_i8(*self).map_err(MoshroomError::IoError)
    }
}

impl MooshroomReadable for u8 {
    fn read(
        mut reader: impl std::io::Read,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<Self> {
        reader.read_u8().map_err(MoshroomError::IoError)
    }
}

impl MooshroomWritable for u8 {
    fn write(
        &self,
        mut writer: impl std::io::Write,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        writer.write_u8(*self).map_err(MoshroomError::IoError)
    }
}

impl MooshroomReadable for String {
    fn read(
        mut reader: impl std::io::Read,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<Self> {
        let len = VarInt::read(&mut reader, version)?;

        let s = {
            let mut buffer: Vec<u8> = vec![0; len.0 as usize];
            reader.read_exact(&mut buffer)?;
            String::from_utf8(buffer).map_err(MoshroomError::InvalidString)?
        };
        Ok(s)
    }
}

impl MooshroomWritable for String {
    fn write(
        &self,
        mut writer: impl std::io::Write,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        VarInt(self.len() as i32).write(&mut writer, version)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T> MooshroomReadable for Vec<T>
where
    T: MooshroomReadable,
{
    fn read(
        mut reader: impl std::io::Read,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<Self> {
        let len = VarInt::read(&mut reader, version)?.0 as usize;

        let mut buffer = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(T::read(&mut reader, version)?);
        }
        Ok(buffer)
    }
}

impl<T> MooshroomWritable for Vec<T>
where
    T: MooshroomWritable,
{
    fn write(
        &self,
        mut writer: impl std::io::Write,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        VarInt(self.len() as i32).write(&mut writer, version)?;
        for p in self.iter() {
            p.write(&mut writer, version)?;
        }
        Ok(())
    }
}

impl<T, const N: usize> MooshroomReadable for [T; N]
where
    T: MooshroomReadable + Sized,
{
    fn read(
        mut reader: impl std::io::Read,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<Self> {
        unsafe {
            let mut buffer: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
            for i in buffer.iter_mut() {
                i.write(T::read(&mut reader, version)?);
            }
            Ok(buffer.as_ptr().cast::<[T; N]>().read())
        }
    }
}

impl<T, const N: usize> MooshroomWritable for [T; N]
where
    T: MooshroomWritable,
{
    fn write(
        &self,
        mut writer: impl std::io::Write,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        for i in self.iter() {
            i.write(&mut writer, version)?;
        }
        Ok(())
    }
}

#[cfg(feature = "uuid")]
impl MooshroomReadable for uuid::Uuid {
    fn read(
        reader: impl std::io::Read,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<Self> {
        let s = String::read(reader, version)?;
        Ok(uuid::Uuid::parse_str(&s)?)
    }
}

#[cfg(feature = "uuid")]
impl MooshroomWritable for uuid::Uuid {
    fn write(
        &self,
        writer: impl std::io::Write,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        let s = self.to_string();
        s.write(writer, version)
    }
}
