pub mod client;
pub mod codec;
pub mod containers;
pub mod proto;
pub mod server;

pub use mooshroom_core as core;
pub use mooshroom_core::ProtocolVersion;
pub use mooshroom_macros as macros;

pub mod prelude {
    pub use mooshroom_core::{
        error::MoshroomError,
        io::{MooshroomCommand, MooshroomPacket, MooshroomReadable, MooshroomWritable},
    };
}
