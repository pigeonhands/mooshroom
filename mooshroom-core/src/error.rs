use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoshroomError {
    #[error("Problem with io")]
    IoError(std::io::Error),
    #[error("VarInt may be up to 5 bytes")]
    VarIntTooLong,
    #[error("VarInt may be up to 5 bytes")]
    WrongPacket(crate::varint::VarInt),
    #[error("VarInt may be up to 5 bytes")]
    InvalidHandshakeState(i32),
    #[error("invalid string")]
    InvalidString(std::string::FromUtf8Error),
    #[error("invalid json. {0}")]
    InvalidJson(String),
    #[error("Unexpected packet. Expected {0}, got {1}")]
    UnexpectedPacket(i32, i32),

    #[cfg(feature = "uuid")]
    #[error("Invalid uuid. {0}")]
    UuidError(uuid::Error),
}

impl From<std::io::Error> for MoshroomError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Error> for MoshroomError {
    fn from(e: uuid::Error) -> Self {
        Self::UuidError(e)
    }
}

pub type Result<T> = std::result::Result<T, MoshroomError>;
