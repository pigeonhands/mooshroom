pub mod client;
pub mod server;
pub mod containers;
pub mod codec;

pub use mooshroom_core as core;
pub use mooshroom_macros as macros;
pub use mooshroom_core::ProtocolVersion;

pub mod prelude {
    pub use mooshroom_core::io::{MooshroomReadable, MooshroomWritable, MooshroomCommand, MooshroomPacket};
    pub use mooshroom_core::error::MoshroomError;
}