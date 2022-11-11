pub mod codec;
pub mod connection;

use codec::MooshroomCodec;

use crate::core::{error::*, io::*};

pub struct MooshroomProto<T> {
    inner: T,
    pub codec: MooshroomCodec<DEFAULT_PROTOCAL_VERSION>,
}

impl<T> MooshroomProto<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            codec: MooshroomCodec::new(),
        }
    }

    pub const fn protocal_version(&self) -> i32 {
        self.codec.protocal_version()
    }
}

impl<R> MooshroomProto<R>
where
    R: std::io::Read,
{
    pub fn buffer_read(&mut self) -> Result<()> {
        let mut buffer = [0; 1024];
        loop {
            let n = self.inner.read(&mut buffer)?;
            if n > 0 {
                self.codec.add_bytes(&buffer[..n]);
                return Ok(());
            }
        }
    }
    pub fn read_packet<T: MooshroomPacket<DEFAULT_PROTOCAL_VERSION>>(&mut self) -> Result<T> {
        let mut buffer = [0; 1024];
        loop {
            if let Some(p) = self.codec.read_packet()? {
                return Ok(p);
            }
            let n = self.inner.read(&mut buffer)?;
            if n > 0 {
                self.codec.add_bytes(&buffer[..n]);
            }
        }
    }

    pub fn read_one_of<T: MooshroomCollection<DEFAULT_PROTOCAL_VERSION>>(&mut self) -> Result<T> {
        let mut buffer = [0; 1024];
        loop {
            if let Some(p) = self.codec.read_one_of()? {
                return Ok(p);
            }
            let n = self.inner.read(&mut buffer)?;
            if n > 0 {
                self.codec.add_bytes(&buffer[..n]);
            }
        }
    }
}

impl<T> MooshroomProto<T>
where
    T: std::io::Write,
{
    pub fn write_packet(
        &mut self,
        p: &impl MooshroomPacket<DEFAULT_PROTOCAL_VERSION>,
    ) -> Result<()> {
        let bytes = self.codec.encode(p)?;
        self.inner.write_all(&bytes)?;
        Ok(())
    }
}

impl<T> MooshroomProto<T>
where
    T: std::io::Read + std::io::Write,
{
    pub fn send_command<P: MooshroomCommand<DEFAULT_PROTOCAL_VERSION>>(
        &mut self,
        p: &P,
    ) -> Result<P::Response> {
        self.write_packet(p)?;
        self.read_packet()
    }
}
