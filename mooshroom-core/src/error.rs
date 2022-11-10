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
    #[error("invalid json")]
    InvalidJson,
    #[error("Unexpected packet. Got {0}, expecting {1}")]
    UnexpectedPacket(i32, i32)
}

impl From<std::io::Error> for MoshroomError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

pub type Result<T> = std::result::Result<T, MoshroomError>;
