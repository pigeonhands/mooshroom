mod bitfield;
mod collection;
mod default;
mod enum_value;
mod mooshroom;
mod updatable;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Mooshroom, attributes(packet_id, response, value_type, read, id))]
pub fn mooshroom_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = mooshroom::impl_mooshroom_packet(&ast);

    gen.into()
}

#[proc_macro_derive(MooshroomCollection, attributes(id, id_range))]
pub fn collection_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = collection::impl_collection(&ast);

    gen.into()
}

#[proc_macro_derive(MooshroomUpdatable, attributes(update_using, from, extends))]
pub fn struct_updatable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = updatable::impl_mooshroom_updatable(&ast);

    gen.into()
}

#[proc_macro_derive(DefaultInline, attributes(default))]
pub fn default_inline(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = default::impl_mooshroom_default_inline(&ast);

    gen.into()
}

#[proc_macro_derive(MooshroomBitfield, attributes(value_type, mask))]
pub fn impl_bitflag(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = bitfield::impl_mooshroom_bitfield(&ast);

    gen.into()
}
