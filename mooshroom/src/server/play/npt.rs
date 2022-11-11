use cesu8::from_cesu8;
use mooshroom_core::{
    error::MoshroomError,
    io::{
        MooshroomReadProto,
        MooshroomReadable,
        MooshroomWritable,
        Protocal,
        DEFAULT_PROTOCAL_VERSION,
    },
};

#[derive(Debug, Clone, Default)]
pub struct NptCompound(NptNamedTag<DEFAULT_PROTOCAL_VERSION>);

impl<const PV: Protocal> MooshroomReadable<PV> for NptCompound {
    fn read(mut reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let d = NptTagData::read_compound_body(&mut reader)?;
        Ok(Self(d))
    }
}

impl<const PV: Protocal> MooshroomWritable<PV> for NptCompound {
    fn write(&self, _: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NptTagDataDefault(NptTagData<DEFAULT_PROTOCAL_VERSION>);

impl<const PV: Protocal> MooshroomReadable<PV> for NptTagDataDefault {
    fn read(mut reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let ty = <u8 as MooshroomReadable<PV>>::read(&mut reader)?;
        Ok(Self(NptTagData::read_type(ty, &mut reader)?))
    }
}

impl<const PV: Protocal> MooshroomWritable<PV> for NptTagDataDefault {
    fn write(&self, _: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NptNamedTag<const PV: Protocal>(String, NptTagData<PV>);

#[derive(Debug, Clone, Default)]
pub enum NptTagData<const PV: Protocal> {
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
    List(Vec<NptTagData<PV>>),
    Compount(Vec<NptNamedTag<PV>>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl<const PV: Protocal> MooshroomReadable<PV> for NptTagData<PV> {
    fn read(mut reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let ty = <u8 as MooshroomReadable<PV>>::read(&mut reader)?;
        Self::read_type(ty, &mut reader)
    }
}

impl<const PV: Protocal> MooshroomWritable<PV> for NptTagData<PV> {
    fn write(&self, _: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        todo!()
    }
}

impl<const PV: Protocal> NptTagData<PV> {
    fn read_type(ty: u8, reader: &mut impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let r = match ty {
            0 => Self::End,
            1 => Self::Byte(i8::read_proto::<PV>(reader)?),
            2 => Self::Short(i16::read_proto::<PV>(reader)?),
            3 => Self::Int(i32::read_proto::<PV>(reader)?),
            4 => Self::Long(i64::read_proto::<PV>(reader)?),
            5 => Self::Float(f32::read_proto::<PV>(reader)?),
            6 => Self::Double(f64::read_proto::<PV>(reader)?),
            7 => Self::ByteArray(Self::read_byte_array(reader)?),
            8 => Self::String(Self::read_string(reader)?),
            9 => Self::List(Self::read_list(reader)?),
            10 => Self::read_compound(reader)?,
            11 => Self::IntArray(Self::read_array(reader)?),
            12 => Self::LongArray(Self::read_array(reader)?),
            _ => unimplemented!(),
        };
        Ok(r)
    }
    fn read_byte_array(
        mut reader: &mut impl std::io::Read,
    ) -> mooshroom_core::error::Result<Vec<u8>> {
        let len = <i32 as MooshroomReadable<PV>>::read(&mut reader)?;
        let mut buffer = vec![0; len as usize];
        reader.read_exact(&mut buffer[..])?;
        Ok(buffer)
    }
    fn read_string(mut reader: &mut impl std::io::Read) -> mooshroom_core::error::Result<String> {
        let len = u16::read_proto::<PV>(&mut reader)?;
        let mut buffer = vec![0; len as usize];
        reader.read_exact(&mut buffer[..])?;
        let s = from_cesu8(&buffer).map_err(|_| MoshroomError::InvalidNbtTag(8))?;
        Ok(s.into_owned())
    }
    fn read_list(
        mut reader: &mut impl std::io::Read,
    ) -> mooshroom_core::error::Result<Vec<NptTagData<PV>>> {
        let ty = <u8 as MooshroomReadable<PV>>::read(&mut reader)?;
        let len = <u32 as MooshroomReadable<PV>>::read(&mut reader)?;
        let mut items = Vec::with_capacity(len as usize);
        for _ in 0..len {
            items.push(Self::read_type(ty, reader)?);
        }
        Ok(items)
    }
    fn read_compound_body(
        mut reader: &mut impl std::io::Read,
    ) -> mooshroom_core::error::Result<NptNamedTag<PV>> {
        let ty = <u8 as MooshroomReadable<PV>>::read(&mut reader)?;
        if ty == 0 {
            Ok(NptNamedTag(String::new(), Self::End))
        } else {
            let cp_name = Self::read_string(&mut reader)?;
            Ok(NptNamedTag(cp_name, Self::read_type(ty, reader)?))
        }
    }
    fn read_compound(reader: &mut impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let mut items = Vec::new();
        loop {
            let body = Self::read_compound_body(reader)?;
            if matches!(body.1, Self::End) {
                break;
            }
            items.push(body);
        }
        Ok(Self::Compount(items))
    }

    fn read_array<D: MooshroomReadable<PV>>(
        mut reader: &mut impl std::io::Read,
    ) -> mooshroom_core::error::Result<Vec<D>> {
        let len = <i32 as MooshroomReadable<PV>>::read(&mut reader)?;
        let mut items = Vec::with_capacity(len as usize);
        for _ in 0..len {
            items.push(D::read(&mut reader)?);
        }
        Ok(items)
    }
}