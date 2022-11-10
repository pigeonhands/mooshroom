pub mod error;
pub mod io;
mod primitives;
pub mod varint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    V1_19_2 = 760,
    V1_16_5 = 754,
    V1_13_2 = 404,
}
