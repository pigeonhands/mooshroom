use quote::quote;
use syn::{Attribute, Data, DataStruct, Fields, FieldsNamed, LitInt};

#[derive(Default)]
struct MooshroomAttrs {
    packet_id: Option<i32>,
    response: Option<syn::Ident>,
}

impl MooshroomAttrs {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut ma = Self::default();
        for attr in attributes {
            if attr.path.is_ident("packet_id") {
                ma.packet_id = match attr
                    .parse_args()
                    .ok()
                    .map(|l: LitInt| l.base10_parse().unwrap())
                {
                    Some(s) => Some(s),
                    None => panic!("packet_id must be i32"),
                }
            } else if attr.path.is_ident("response") {
                ma.response = match attr.parse_args() {
                    Ok(r) => r,
                    Err(e) => panic!("response must be ident. {}", e),
                }
            }
        }
        ma
    }
}

pub fn impl_mooshroom_packet(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        Data::Struct(s) => impl_mooshroom_packet_struct(ast, s),
        _ => unimplemented!("impl_mooshroom_packet"),
    }
}

fn impl_mooshroom_packet_struct(
    ast: &syn::DeriveInput,
    data: &DataStruct,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let attrs = MooshroomAttrs::parse(&ast.attrs);

    let (read, write) = match &data.fields {
        Fields::Named(fields) => handle_named_fields(fields),
        Fields::Unit => (
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        ),
        _ => unimplemented!("impl_mooshroom_packet_struct"),
    };

    let packet = attrs.packet_id.map(|id| {
        quote! {
            #[automatically_derived]
            impl ::mooshroom_core::io::MooshroomPacket for #name {
                const PACKET_ID : ::mooshroom_core::varint::VarInt = ::mooshroom_core::varint::VarInt(#id);
            }
        }
    });

    let response = attrs.response.map(|r| {
        quote! {
            #[automatically_derived]
            impl ::mooshroom_core::io::MooshroomCommand for #name {
                type Response = #r;
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl ::mooshroom_core::io::MooshroomReadable for #name {
            fn read(mut reader: impl ::std::io::Read, version: ::mooshroom_core::ProtocolVersion) -> ::mooshroom_core::error::Result<Self> {
                Ok(
                    Self{
                        #read
                    }
                )
            }
        }

        #[automatically_derived]
        impl ::mooshroom_core::io::MooshroomWritable for #name {
            fn write(&self, mut writer: impl ::std::io::Write, version: ::mooshroom_core::ProtocolVersion) -> ::mooshroom_core::error::Result<()> {
                #write
                Ok(())
            }
        }

        #packet
        #response
    }
}

fn handle_named_fields(
    fields: &FieldsNamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let read: Vec<proc_macro2::TokenStream> = fields
        .named
        .iter()
        .filter_map(|it| {
            it.ident.as_ref().map(|i| {
                let ident = i;
                let ty = &it.ty;

                quote! {
                    #ident: <#ty as ::mooshroom_core::io::MooshroomReadable>::read(&mut reader, version)?
                }
            })
        })
        .collect();

    let write: Vec<proc_macro2::TokenStream> = fields
        .named
        .iter()
        .filter_map(|it| {
            it.ident.as_ref().map(|i| {
                let ident = i;

                quote! {
                    ::mooshroom_core::io::MooshroomWritable::write(&self.#ident, &mut writer, version)?;
                }
            })
        })
        .collect();

    let read = quote! {
        #( #read ), *
    };
    let write = quote! {
        #( #write ) *
    };

    (read, write)
}
