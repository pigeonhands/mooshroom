use std::io::{Read};

use bytes::BytesMut;
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use mooshroom_core::{varint::VarInt, ProtocolVersion};

use crate::core::{
    error::{MoshroomError, Result},
    io::*,
};

pub enum PacketBody<'a> {
    Owned(BytesMut),
    Borrowed(&'a [u8]),
}
impl<'a> AsRef<[u8]> for PacketBody<'a> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Owned(o) => o.as_ref(),
            Self::Borrowed(b) => b,
        }
    }
}

pub struct PacketData<'a> {
    pub packet_id: VarInt,
    pub body: PacketBody<'a>,
}

pub struct MooshroomCodec {
    protocol: ProtocolVersion,

    compression: Option<i32>,
    write_buffer: Vec<u8>,
    compress_buffer: Vec<u8>,
    rx_buffer: BytesMut,
}

impl MooshroomCodec {
    pub fn new() -> Self {
        Self {
            write_buffer: Vec::new(),
            compress_buffer: Vec::new(),
            protocol: ProtocolVersion::V1_19_2,
            compression: None,

            rx_buffer: BytesMut::new(),
        }
    }
}

impl MooshroomCodec {
    pub fn set_compression(&mut self, th: i32) {
        if th < 0 {
            self.compression = None;
        }else{
            self.compression = Some(th);
        }
    }
    pub fn finalize_compressed(&mut self) -> Result<Vec<u8>> {
        let compressed_bytes : &[u8] = {
            let mut compress = ZlibEncoder::new(self.write_buffer.as_slice(), Compression::default());
            compress.read_to_end(&mut self.compress_buffer)?;
            self.compress_buffer.as_ref()
        };
        let mut uncompressed_len_buffer = [0; 10];
        let mut total_len_buffer = [0; 10];

        let uncompressed_len = {
            let n = VarInt(self.write_buffer.len() as i32)
                .write_with_size(&mut uncompressed_len_buffer[..], self.protocol)?;
            &uncompressed_len_buffer[..n]
        };
        let total_len = {
            let n = VarInt((uncompressed_len.len() + compressed_bytes.len()) as i32)
                .write_with_size(&mut total_len_buffer[..], self.protocol)?;
            &total_len_buffer[..n]
        };

        let mut buffer =
            Vec::with_capacity(total_len.len() + uncompressed_len.len() + compressed_bytes.len());
        buffer.extend_from_slice(total_len);
        buffer.extend_from_slice(uncompressed_len);
        buffer.extend_from_slice(compressed_bytes);
        Ok(buffer)
    }

    fn finalize_uncompressed(&mut self) -> Result<Vec<u8>> {
        let mut len_buffer = [0; 10];
        let len_bytes = VarInt(self.write_buffer.len() as i32)
            .write_with_size(&mut len_buffer[..], self.protocol)?;

        let mut buffer = Vec::with_capacity(self.write_buffer.len() + len_bytes);
        buffer.extend_from_slice(&len_buffer[..len_bytes]);
        buffer.extend_from_slice(&self.write_buffer);
        Ok(buffer)
    }

    pub fn encode<T: MooshroomPacket>(&mut self, packet: &T) -> Result<Vec<u8>> {
        self.write_buffer.clear();
        self.compress_buffer.clear();

        T::PACKET_ID.write(&mut self.write_buffer, self.protocol)?;
        packet.write(&mut self.write_buffer, self.protocol)?;

        let buffer = if let Some(_) = self.compression {
            self.finalize_compressed()?
        } else {
            self.finalize_uncompressed()?
        };

        Ok(buffer)
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) {
        self.rx_buffer.extend(bytes);
    }

    pub fn peek_packet(&mut self) -> Option<(VarInt, usize)> {
        let (length, lenght_bytes_n) =
            VarInt::read_with_size(&self.rx_buffer, self.protocol).ok()?;

        let required_size = lenght_bytes_n + length.0 as usize;
        if self.rx_buffer.len() < required_size {
            return None;
        } else {
            Some((length, lenght_bytes_n))
        }
    }

    pub fn read_packet_data(&mut self) -> Result<Option<PacketData>> {
        let (length, lenght_bytes_n) = match self.peek_packet() {
            Some((l, n)) => (l, n),
            None => return Ok(None),
        };


        let required_size = lenght_bytes_n + length.0 as usize;

        let mut raw_data = {
            let mut spl = self.rx_buffer.split_off(required_size);
            std::mem::swap(&mut spl, &mut self.rx_buffer);
            spl = spl.split_off(lenght_bytes_n);
            spl
        };

        let decompressed_size = if self.compression.is_some() {
            let (decompressed_size, decompressed_size_n) =
                VarInt::read_with_size(&raw_data, self.protocol)?;
            raw_data = raw_data.split_off(decompressed_size_n);
            if decompressed_size.0 > 0 {
                Some(decompressed_size.0 as usize)
            }else{
                None
            }
        }else{
            None
        };

        if let Some(decompressed_size) = decompressed_size {
            
            self.compress_buffer.clear();
            self.compress_buffer
                .reserve_exact(decompressed_size);

            let decompressed_bytes = {
                let mut decompress =
                    ZlibDecoder::new(&raw_data[..]);
                decompress.read_to_end(&mut self.compress_buffer)?;
                self.compress_buffer.as_ref()
            };


            let (packet_id, packet_id_n) =
                VarInt::read_with_size(&decompressed_bytes, self.protocol)?;
            Ok(Some(PacketData {
                packet_id: packet_id,
                body: PacketBody::Borrowed(&decompressed_bytes[packet_id_n..]),
            }))
        } else {
            let (packet_id, packet_id_n) = VarInt::read_with_size(&raw_data, self.protocol)?;
            let raw_data = raw_data.split_off(packet_id_n);

            Ok(Some(PacketData {
                packet_id: packet_id,
                body: PacketBody::Owned(raw_data),
            }))
        }
    }

    pub fn read_packet<'a, P: MooshroomPacket>(&mut self) -> Result<Option<P>> {
        let protocol = self.protocol;
        let data = match self.read_packet_data()? {
            Some(e) => e,
            None => return Ok(None),
        };

        if P::PACKET_ID != data.packet_id {
            return Err(MoshroomError::UnexpectedPacket(
                P::PACKET_ID.0,
                data.packet_id.0,
            ));
        }
        let mut b = data.body.as_ref();
        P::read(&mut b, protocol).map(Some)
    }

    pub fn read_one_of<P: MooshroomCollection>(&mut self) -> Result<Option<P>> {
        let protocol = self.protocol;
        let data = match self.read_packet_data()? {
            Some(e) => e,
            None => return Ok(None),
        };
        let resp = P::read_one_of(data.packet_id, data.body.as_ref(), protocol)?;
        Ok(Some(resp))
    }
}
