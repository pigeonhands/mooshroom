use quote::quote;
use syn::{Data, DataEnum, Fields};

pub fn impl_collection(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        Data::Enum(s) => impl_collection_enum(ast, s),
        _ => unimplemented!("impl_mooshroom_packet"),
    }
}

fn impl_collection_enum(ast: &syn::DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let selector = data
        .variants
        .iter()
        .filter_map(|f| {
            if let Fields::Unnamed(un) = &f.fields {
                Some((&f.ident, un))
            } else {
                None
            }
        })
        .map(|(name, data)| {
            let f = data.unnamed.first().unwrap();
            let ty = &f.ty;
            quote! {
                <#ty as ::mooshroom_core::io::MooshroomPacket<PV>>::PACKET_ID => Ok(Self::#name(<#ty as ::mooshroom_core::io::MooshroomReadable<PV>>::read(reader)?)),
            }
        });

    quote! {
        #[automatically_derived]
        impl<const PV: ::mooshroom_core::io::Protocal> ::mooshroom_core::io::MooshroomCollection<PV> for #name {
            fn read_one_of(id: ::mooshroom_core::varint::VarInt, mut reader: impl ::std::io::Read) -> ::mooshroom_core::error::Result<Self>{
                match id {
                    #( #selector ) *
                    i => Err(::mooshroom_core::error::MoshroomError::NotInCollection(i.0))
                }
            }
        }
    }
}
