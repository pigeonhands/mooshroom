use thiserror::Error;

#[derive(Error, Debug)]
pub enum MooshroomError {
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
    #[error("Packet id {0} is not in collection")]
    NotInCollection(i32),
    #[error("Failed to parse nbt tag of type {0}")]
    InvalidNbtTag(u8),
    #[error("Invalid enum variant {0}")]
    InvalidEnumVariant(i32),
    #[error("Could not find value for id {0} ")]
    InvalidId(String),
    #[error("No Id found for value")]
    NoId,

    #[cfg(feature = "uuid")]
    #[error("Invalid uuid. {0}")]
    UuidError(uuid::Error),

    #[error("Unexpected error: {0}")]
    Other(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for MooshroomError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Error> for MooshroomError {
    fn from(e: uuid::Error) -> Self {
        Self::UuidError(e)
    }
}

pub type Result<T> = std::result::Result<T, MooshroomError>;
