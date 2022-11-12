use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

use super::io::{MooshroomReadable, MooshroomWritable};
use crate::error::{MooshroomError, Result};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl VarInt {
    pub fn new(i: i32) -> Self {
        Self(i)
    }
    pub fn as_i32(&self) -> i32 {
        self.0
    }
    pub fn read_with_size<const PV: usize>(buffer: &[u8]) -> Result<(VarInt, usize)> {
        let mut cur = std::io::Cursor::new(buffer);
        let i = <Self as MooshroomReadable<PV>>::read(&mut cur)?;
        Ok((i, cur.position() as usize))
    }
    pub fn write_with_size<const PV: usize>(&self, buffer: &mut [u8]) -> Result<usize> {
        let mut cur = std::io::Cursor::new(buffer);
        <Self as MooshroomWritable<PV>>::write(&self, &mut cur)?;
        Ok(cur.position() as usize)
    }
}

impl PartialEq<i32> for VarInt {
    fn eq(&self, i: &i32) -> bool {
        self.0 == *i
    }
}

impl From<i32> for VarInt {
    fn from(i: i32) -> Self {
        Self(i)
    }
}

impl AsRef<i32> for VarInt {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl<const PV: usize> MooshroomReadable<PV> for VarInt {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            let read = reader.read_u8()?;
            let value = i32::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if num_read > 5 {
                return Err(MooshroomError::VarIntTooLong);
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(Self(result))
    }
}

impl<const PV: usize> MooshroomWritable<PV> for VarInt {
    fn write(&self, writer: &mut impl io::Write) -> Result<()> {
        let mut x = self.0 as u32;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            writer.write_all(&[temp])?;

            if x == 0 {
                break;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl<const PV: usize> MooshroomReadable<PV> for VarLong {
    fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            let read = reader.read_u8()?;
            let value = i64::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if read & 0b1000_0000 == 0 {
                break;
            }

            if num_read > 10 {
                return Err(MooshroomError::VarIntTooLong);
            }
        }
        Ok(Self(result))
    }
}

impl<const PV: usize> MooshroomWritable<PV> for VarLong {
    fn write(&self, writer: &mut impl io::Write) -> Result<()> {
        let mut x = self.0 as u64;
        writer
            .write_u8(((x & 0b0111_1111) | (0b1000_0000 * ((x >> 7 != 0) as u64))) as u8)
            .map_err(MooshroomError::IoError)?;
        x >>= 7;
        loop {
            if x == 0 {
                break;
            }
            let mut temp = (x & 0b0111_1111) as u8;

            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            writer.write_u8(temp).map_err(MooshroomError::IoError)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::{MooshroomReadProto, MooshroomWriteProto, DEFAULT_PROTOCAL_VERSION};

    #[test]
    fn varint_read() {
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x0].as_ref()).unwrap(),
            VarInt(0)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x1].as_ref()).unwrap(),
            VarInt(1)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x2].as_ref()).unwrap(),
            VarInt(2)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x7f].as_ref()).unwrap(),
            VarInt(127)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x80, 0x01].as_ref()).unwrap(),
            VarInt(128)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0xff, 0x01].as_ref()).unwrap(),
            VarInt(255)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0xdd, 0xc7, 0x01].as_ref())
                .unwrap(),
            VarInt(25565)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0xff, 0xff, 0x7f].as_ref())
                .unwrap(),
            VarInt(2097151)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0xff, 0xff, 0xff, 0xff, 0x07].as_ref()
            )
            .unwrap(),
            VarInt(2147483647)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0xff, 0xff, 0xff, 0xff, 0x0f].as_ref()
            )
            .unwrap(),
            VarInt(-1)
        );
        assert_eq!(
            VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0x80, 0x80, 0x80, 0x80, 0x08].as_ref()
            )
            .unwrap(),
            VarInt(-2147483648)
        );
    }

    #[test]
    fn varint_write() {
        fn check_varint<const N: usize>(value: i32, expected: [u8; N]) {
            let mut buffer = Vec::new();
            <VarInt as MooshroomWritable<DEFAULT_PROTOCAL_VERSION>>::write(
                &VarInt(value),
                &mut buffer,
            )
            .unwrap();
            let out: [u8; N] = buffer.try_into().unwrap();

            assert_eq!(expected, out);
        }

        check_varint(0, [0x0]);
        check_varint(1, [0x1]);
        check_varint(2, [0x2]);
        check_varint(127, [0x7f]);
        check_varint(128, [0x80, 0x01]);
        check_varint(255, [0xff, 0x01]);
        check_varint(25565, [0xdd, 0xc7, 0x01]);
        check_varint(2097151, [0xff, 0xff, 0x7f]);
        check_varint(2147483647, [0xff, 0xff, 0xff, 0xff, 0x07]);
        check_varint(-1, [0xff, 0xff, 0xff, 0xff, 0x0f]);
        check_varint(-2147483648, [0x80, 0x80, 0x80, 0x80, 0x08]);
    }

    #[test]
    fn varint_write_read() {
        let tests = [
            0,
            1,
            2,
            127,
            128,
            255,
            25565,
            2097151,
            2147483647,
            -1,
            -2147483648,
        ];
        let mut buffer = Vec::new();
        for i in tests {
            buffer.clear();
            let vi = VarInt(i);
            <VarInt as MooshroomWritable<DEFAULT_PROTOCAL_VERSION>>::write(&vi, &mut buffer)
                .unwrap();
            let vo =
                VarInt::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut buffer.as_slice()).unwrap();
            assert_eq!(vi, vo);
        }
    }

    #[test]
    fn varlong_read() {
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x0].as_ref()).unwrap(),
            VarLong(0)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x1].as_ref()).unwrap(),
            VarLong(1)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x2].as_ref()).unwrap(),
            VarLong(2)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x7f].as_ref()).unwrap(),
            VarLong(127)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0x80, 0x01].as_ref()).unwrap(),
            VarLong(128)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0xff, 0x01].as_ref()).unwrap(),
            VarLong(255)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0xdd, 0xc7, 0x01].as_ref())
                .unwrap(),
            VarLong(25565)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut [0xff, 0xff, 0x7f].as_ref())
                .unwrap(),
            VarLong(2097151)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0xff, 0xff, 0xff, 0xff, 0x07].as_ref()
            )
            .unwrap(),
            VarLong(2147483647)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f].as_ref()
            )
            .unwrap(),
            VarLong(9223372036854775807)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01].as_ref()
            )
            .unwrap(),
            VarLong(-1)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01].as_ref()
            )
            .unwrap(),
            VarLong(-2147483648)
        );
        assert_eq!(
            VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(
                &mut [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01].as_ref()
            )
            .unwrap(),
            VarLong(-9223372036854775808)
        );
    }

    #[test]
    fn varlong_write() {
        fn check_varlong<const N: usize>(value: i64, expected: [u8; N]) {
            let mut buffer = Vec::new();
            <VarLong as MooshroomWritable<DEFAULT_PROTOCAL_VERSION>>::write(
                &VarLong(value),
                &mut buffer,
            )
            .unwrap();
            assert_eq!(buffer.len(), N, "Value: {}", value);
            let out: [u8; N] = buffer.try_into().unwrap();

            assert_eq!(expected, out);
        }

        check_varlong(0, [0x0]);
        check_varlong(1, [0x1]);
        check_varlong(2, [0x2]);
        check_varlong(127, [0x7f]);
        check_varlong(128, [0x80, 0x01]);
        check_varlong(255, [0xff, 0x01]);
        check_varlong(25565, [0xdd, 0xc7, 0x01]);
        check_varlong(2097151, [0xff, 0xff, 0x7f]);
        check_varlong(2147483647, [0xff, 0xff, 0xff, 0xff, 0x07]);
        check_varlong(
            9223372036854775807,
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
        );
        check_varlong(
            -1,
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
        );
        check_varlong(
            -2147483648,
            [0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
        );
        check_varlong(
            -9223372036854775808,
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        );
    }

    #[test]
    fn varlong_write_read() {
        let tests = [
            0,
            1,
            2,
            127,
            128,
            255,
            25565,
            2097151,
            2147483647,
            9223372036854775807,
            -1,
            -2147483648,
            -9223372036854775808,
        ];
        let mut buffer = Vec::new();
        for i in tests {
            buffer.clear();
            let vi = VarLong(i);
            vi.write_proto::<DEFAULT_PROTOCAL_VERSION>(&mut buffer)
                .unwrap();

            let vo =
                VarLong::read_proto::<DEFAULT_PROTOCAL_VERSION>(&mut buffer.as_slice()).unwrap();
            assert_eq!(vi, vo);
        }
    }
}
