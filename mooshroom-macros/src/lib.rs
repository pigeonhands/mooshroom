mod collection;
mod enum_value;
mod mooshroom;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Mooshroom, attributes(packet_id, response, value_type, read))]
pub fn mooshroom_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = mooshroom::impl_mooshroom_packet(&ast);

    gen.into()
}

#[proc_macro_derive(MooshroomCollection)]
pub fn collection_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = collection::impl_collection(&ast);

    gen.into()
}

#[proc_macro_derive(MooshroomEnum, attributes(value_type))]
pub fn enum_value_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = collection::impl_collection(&ast);

    gen.into()
}
