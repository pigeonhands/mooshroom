use std::io::Cursor;

use mooshroom_core::ProtocolVersion;
use mooshroom_core::varint::VarInt;
use bytes::{BytesMut};
use crate::core::io::*;
use crate::core::error::{Result, MoshroomError};

pub struct PacketData {
    pub length_size: usize,
    pub body: BytesMut
}

pub struct MooshroomCodec {
    protocol: ProtocolVersion,

    write_buffer: Vec<u8>,
    rx_buffer: BytesMut,
}

impl MooshroomCodec {
    pub fn new() -> Self{
        Self{
            write_buffer: Vec::new(),
            protocol: ProtocolVersion::V1_16_5,

            rx_buffer: BytesMut::new(),
        }
    }
}

impl MooshroomCodec {
    pub fn encode<T: MooshroomPacket>(&mut self, packet: T) -> Result<Vec<u8>>{
        self.write_buffer.clear();

        T::PACKET_ID.write(&mut self.write_buffer, self.protocol)?;
        packet.write(&mut self.write_buffer, self.protocol)?;

        let mut buffer = {
            let mut len_buffer = [0;10];
            let len_bytes = {
                let len_n = {
                    let mut cur = Cursor::new(&mut len_buffer[..]);
                    let len_varint : VarInt = (self.write_buffer.len() as i32).into();
                    len_varint.write(&mut cur, self.protocol)?;
                    cur.position() as usize
                };
                &len_buffer[..len_n]
            };

            let mut buffer = Vec::with_capacity(self.write_buffer.len() + len_bytes.len());
            buffer.extend_from_slice(len_bytes);
            buffer
        };
        buffer.extend_from_slice(&self.write_buffer);

        Ok(buffer)
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) {
        self.rx_buffer.extend(bytes);
    }

    pub fn read_packet_data_raw(&mut self) -> Option<PacketData> {
        let (length, lenght_bytes_n) = {
            let mut cur = Cursor::new(&mut self.rx_buffer);
            (VarInt::read(&mut cur, self.protocol), cur.position() as usize)
        };
        if let Ok(len) = length {
            let required_size = lenght_bytes_n + len.0 as usize;
            if self.rx_buffer.len() < required_size {
                return None;
            }
            let data = self.rx_buffer.split_off(required_size);
            Some(PacketData{
                body: data,
                length_size: lenght_bytes_n
            })
        }else{
            None
        }
    }

    pub fn read_packet<'a, P: MooshroomPacket>(&mut self) -> Result<Option<P>> {
        let buffer = match self.read_packet_data_raw() {
            Some(e) => e,
            None => return Ok(None)
        };
        let mut cur = Cursor::new(&buffer.body[buffer.length_size..]);
        let packet_id = VarInt::read(&mut cur, self.protocol)?;

        if P::PACKET_ID != packet_id {
            return Err(MoshroomError::UnexpectedPacket(P::PACKET_ID.0, packet_id.0));
        }
        P::read(&mut cur, self.protocol).map(Some)
    }
}


