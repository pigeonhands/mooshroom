use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute,
    Data,
    DataStruct,
    Fields,
    LitInt,
    Token,
};

use crate::enum_value;

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

struct ManualParse {
    func: syn::Ident,
    args: Vec<syn::Ident>,
}

impl Parse for ManualParse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Indent
        let mut vars = Punctuated::<syn::Ident, Token![,]>::parse_terminated(input)?.into_iter();
        Ok(Self {
            func: vars
                .next()
                .expect("Missing func for #[from_context(fn, args..)]"),
            args: vars.collect(),
        })
    }
}

struct FieldAttributes {
    from_context: Option<ManualParse>,
}

impl FieldAttributes {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut from_context: Option<ManualParse> = None;

        for attr in attributes {
            if attr.path.is_ident("parse") {
                from_context = Some(attr.parse_args().unwrap())
            }
        }
        Self { from_context }
    }
}

pub fn impl_mooshroom_packet(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        Data::Struct(s) => impl_mooshroom_packet_struct(ast, s),
        Data::Enum(e) => enum_value::impl_enum(ast, e),
        _ => unimplemented!("impl_mooshroom_packet"),
    }
}

struct PacketStructData<'a> {
    ident: TokenStream,
    ty: &'a syn::Type,
    attrs: FieldAttributes,
}

fn impl_mooshroom_packet_struct(
    ast: &syn::DeriveInput,
    data: &DataStruct,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let attrs = MooshroomAttrs::parse(&ast.attrs);

    let field_data: Vec<PacketStructData> = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|it| {
                let ident = it.ident.as_ref().unwrap();
                PacketStructData {
                    ident: quote!(#ident),
                    ty: &it.ty,
                    attrs: FieldAttributes::parse(&it.attrs),
                }
            })
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, it)| {
                let i = syn::Index::from(i);
                PacketStructData {
                    ident: quote!(#i),
                    ty: &it.ty,
                    attrs: FieldAttributes::parse(&it.attrs),
                }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    };

    let packet_impl = attrs.packet_id.map(|id| {
        quote! {
            #[automatically_derived]
            impl<const PV: usize> ::mooshroom_core::io::MooshroomPacket<PV> for #name {
                const PACKET_ID : ::mooshroom_core::varint::VarInt = ::mooshroom_core::varint::VarInt(#id);
            }
        }
    });

    let response_impl = attrs.response.map(|r| {
        quote! {
            #[automatically_derived]
            impl<const PV: usize> ::mooshroom_core::io::MooshroomCommand<PV> for #name {
                type Response = #r;
            }
        }
    });

    let read_fields : Vec<TokenStream> = field_data.iter().map(|f| {
        let ident = &f.ident;
        let ty = f.ty;
        let r_ident = syn::Ident::new(&format!("r_{}", ident), Span::call_site());
        if let Some(from_ctx) = &f.attrs.from_context{
            let ctx_args : Vec<syn::Ident> = from_ctx.args.iter().map(|a| {
                syn::Ident::new(&format!("r_{}", a), Span::call_site())
            }).collect();
            let ctx_func = &from_ctx.func;
            quote! {
                let #r_ident : #ty = #ctx_func::<PV>(reader #( ,&#ctx_args )* )?;
            }
        }else{
            quote! {
                let #r_ident = <#ty as ::mooshroom_core::io::MooshroomReadable<PV>>::read(reader)?;
            }
        }
    }).collect();

    let idents: Vec<&TokenStream> = field_data.iter().map(|f| &f.ident).collect();
    let r_idents: Vec<syn::Ident> = field_data
        .iter()
        .map(|f| syn::Ident::new(&format!("r_{}", f.ident), Span::call_site()))
        .collect();

    let x = quote! {
        #[automatically_derived]
        impl<const PV: usize> ::mooshroom_core::io::MooshroomReadable<PV> for #name {
            fn read(reader: &mut impl ::std::io::Read) -> ::mooshroom_core::error::Result<Self> {
                #( #read_fields ) *
                Ok(
                    Self{
                    #( #idents: #r_idents ), *
                    }
                )
            }
        }

        #[automatically_derived]
        impl<const PV: usize> ::mooshroom_core::io::MooshroomWritable<PV> for #name {
            fn write(&self, writer: &mut impl ::std::io::Write) -> ::mooshroom_core::error::Result<()> {
                #( ::mooshroom_core::io::MooshroomWritable::<PV>::write(&self.#idents, writer)?; ) *
                Ok(())
            }
        }

        #packet_impl
        #response_impl
    };
    x
}
