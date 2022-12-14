pub mod client;
pub mod containers;
pub mod proto;
pub mod server;
pub mod shared;
pub mod types;
pub use mooshroom_core as core;
pub use mooshroom_core::ProtocolVersion;
pub use mooshroom_macros as macros;

pub mod prelude {
    pub use mooshroom_core::{
        error::MooshroomError,
        io::{MooshroomCommand, MooshroomPacket, MooshroomReadable, MooshroomWritable},
    };
}
