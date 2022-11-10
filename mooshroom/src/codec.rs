use std::io::Cursor;

use bytes::BytesMut;
use mooshroom_core::{varint::VarInt, ProtocolVersion};

use crate::core::{
    error::{MoshroomError, Result},
    io::*,
};

pub struct PacketData {
    pub packet_id: VarInt,
    pub body: BytesMut,
}

pub struct MooshroomCodec {
    protocol: ProtocolVersion,

    write_buffer: Vec<u8>,
    rx_buffer: BytesMut,
}

impl MooshroomCodec {
    pub fn new() -> Self {
        Self {
            write_buffer: Vec::new(),
            protocol: ProtocolVersion::V1_13_2,

            rx_buffer: BytesMut::new(),
        }
    }
}

impl MooshroomCodec {
    pub fn encode<T: MooshroomPacket>(&mut self, packet: &T) -> Result<Vec<u8>> {
        self.write_buffer.clear();

        T::PACKET_ID.write(&mut self.write_buffer, self.protocol)?;
        packet.write(&mut self.write_buffer, self.protocol)?;

        let mut buffer = {
            let mut len_buffer = [0; 10];
            let len_bytes = {
                let len_n = {
                    let mut cur = Cursor::new(&mut len_buffer[..]);
                    let len_varint: VarInt = (self.write_buffer.len() as i32).into();
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

        let mut data = {
            let mut spl = self.rx_buffer.split_off(required_size);
            std::mem::swap(&mut spl, &mut self.rx_buffer);
            spl
        };
        //TODO! encryption/compression

        let (packet_id, packet_id_n) = VarInt::read_with_size(&data, self.protocol)?;
        let data = data.split_off(packet_id_n);

        Ok(Some(PacketData {
            packet_id: packet_id,
            body: data,
        }))
    }

    pub fn read_packet<'a, P: MooshroomPacket>(&mut self) -> Result<Option<P>> {
        let data = match self.read_packet_data()? {
            Some(e) => e,
            None => return Ok(None),
        };

        if P::PACKET_ID != data.packet_id {
            return Err(MoshroomError::UnexpectedPacket(P::PACKET_ID.0, data.packet_id.0));
        }
        let mut b = data.body.as_ref();
        P::read(&mut b, self.protocol).map(Some)
    }
}
