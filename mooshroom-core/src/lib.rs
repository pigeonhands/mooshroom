pub mod error;
pub mod io;
mod primitives;
pub mod varint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    V1_20 = 760,
    V1_16_5 = 754
}