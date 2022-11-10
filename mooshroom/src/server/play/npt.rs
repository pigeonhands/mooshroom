use flate2::read;
use mooshroom_core::io::{MooshroomReadable, MooshroomWritable};
use mooshroom_core::error::MoshroomError;
use cesu8::from_cesu8;

#[derive(Debug, Clone, Default)]
pub struct NptCompound(NptNamedTag);

impl MooshroomReadable for NptCompound {
    fn read(mut reader: impl std::io::Read, version: mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<Self> {
        let d = NptTagData::read_compound_body(&mut reader, version)?;
        Ok(Self(d))
    }
}

impl MooshroomWritable for NptCompound {
    fn write(&self, writer: impl std::io::Write, version: mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<()> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NptNamedTag(String, NptTagData);

#[derive(Debug, Clone, Default)]
pub enum NptTagData {
    #[default]
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NptTagData>),
    Compount(Vec<NptNamedTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

impl NptTagData {
    fn read_type(ty:u8, reader: &mut impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<Self> {
        let r = match ty {
            0 => Self::End,
            1 => Self::Byte(i8::read(reader, version)?),
            2 => Self::Short(i16::read(reader, version)?),
            3 => Self::Int(i32::read(reader, version)?),
            4 => Self::Long(i64::read(reader, version)?),
            5 => Self::Float(f32::read(reader, version)?),
            6 => Self::Double(f64::read(reader, version)?),
            7 => Self::ByteArray(Self::read_byte_array(reader, version)?),
            8 => Self::String(Self::read_string(reader, version)?)
            ,
            9 => Self::List(Self::read_list(reader, version)?),
            10 => Self::read_compound(reader, version)?,
            11 => Self::IntArray(Self::read_array(reader, version)?),
            12 => Self::LongArray(Self::read_array(reader, version)?),
            _ => unimplemented!()
        };
        Ok(r)
    }
    fn read_byte_array(mut reader: &mut impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<Vec<u8>>{
        let len = i32::read(&mut reader, version)?;
        let mut buffer = vec![0; len as usize];
        reader.read_exact(&mut buffer[..])?;
        Ok(buffer)
    }
    fn read_string(mut reader: &mut impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<String>{
        let len = u16::read(&mut reader, version)?;
        let mut buffer = vec![0;len as usize];
        reader.read_exact(&mut buffer[..])?;
        let s = from_cesu8(&buffer).map_err(|_| MoshroomError::InvalidNbtTag(8))?;

        Ok(s.into_owned())
    }
    fn read_list(mut reader: &mut impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<Vec<NptTagData>>{
        let ty = u8::read(&mut reader, version)?;
        let len = i32::read(&mut reader, version)?;
        let mut items = Vec::with_capacity(len as usize);
        for _ in 0..len {
            items.push(Self::read_type(ty, reader, version)?);
        }
        Ok(items)
    }
    fn read_compound_body(mut reader: &mut impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<NptNamedTag> {
        let ty = u8::read(&mut reader,version)?;
        if ty == 0 {
            Ok(NptNamedTag(String::new(), Self::End))
        }else{
            let cp_name = Self::read_string(&mut reader, version)?;
            Ok(NptNamedTag(cp_name, Self::read_type(ty, reader, version)?))
        }
    }
    fn read_compound(reader: &mut impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<Self>{
       
        let mut items = Vec::new();
        loop {
            let body = Self::read_compound_body(reader, version)?;
            if matches!(body.1, Self::End) {
                break;
            }
            items.push(body);
        }
        Ok(Self::Compount(items))
    }

    fn read_array<D: MooshroomReadable>(mut reader: &mut impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<Vec<D>> {
        let len =  i32::read(&mut reader, version)?;
        let mut items = Vec::with_capacity(len as usize);
        for _ in 0..len {
            items.push(D::read(&mut reader, version)?);
        }
        Ok(items)
    }
}
impl MooshroomReadable for NptTagData {
    fn read(mut reader: impl std::io::Read, version : mooshroom_core::ProtocolVersion) -> mooshroom_core::error::Result<Self> {
        let ty = u8::read(&mut reader, version)?;
        Self::read_type(ty, &mut reader, version)
    }
}


