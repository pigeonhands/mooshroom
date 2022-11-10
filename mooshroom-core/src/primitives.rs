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
                fn read(reader: &mut impl std::io::Read, _: crate::ProtocolVersion) -> crate::error::Result<Self> {
                    reader.[<  read_ $e >]::<BigEndian>().map_err(MoshroomError::IoError)
                }
            }

            impl MooshroomWritable for $e {
                fn write(
                    &self,
                    writer: &mut impl std::io::Write,
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

impl MooshroomReadable for i8 {
    fn read(reader: &mut impl std::io::Read, _: crate::ProtocolVersion) -> crate::error::Result<Self>{
        reader.read_i8().map_err(MoshroomError::IoError)
    }
}

impl MooshroomWritable for i8 {
    fn write(
        &self,
        writer: &mut impl std::io::Write,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        writer.write_i8(*self).map_err(MoshroomError::IoError)
    }
}

impl MooshroomReadable for u8 {
    fn read(reader: &mut impl std::io::Read, _: crate::ProtocolVersion) -> crate::error::Result<Self>{
        reader.read_u8().map_err(MoshroomError::IoError)
    }
}

impl MooshroomWritable for u8 {
    fn write(
        &self,
        writer: &mut impl std::io::Write,
        _: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        writer.write_u8(*self).map_err(MoshroomError::IoError)
    }
}

impl MooshroomReadable for String{
    fn read(reader: &mut impl std::io::Read, version: crate::ProtocolVersion) -> crate::error::Result<Self>{
        let len = VarInt::read(reader, version)?;

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
        writer: &mut impl std::io::Write,
        version: crate::ProtocolVersion,
    ) -> crate::error::Result<()> {
        VarInt(self.len() as i32).write(writer, version)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}
