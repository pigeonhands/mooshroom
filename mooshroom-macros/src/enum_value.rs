use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DataEnum, Fields};

struct EnumAttributes {
    read_type: syn::Ident,
}

impl EnumAttributes {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut read_type: Option<syn::Ident> = None;

        for attr in attributes {
            if attr.path.is_ident("repr") && read_type.is_none() {
                read_type = attr.parse_args().unwrap()
            } else if attr.path.is_ident("value_type") {
                read_type = attr.parse_args().expect("#[value_type(T)] must be a type")
            }
        }
        Self {
            read_type: read_type.expect("Must have #[repr] and optinally #[value_type(T)]"),
        }
    }
}

#[derive(Clone)]
struct EnumFieldAttributes {
    pub id: Option<syn::LitStr>,
}

impl EnumFieldAttributes {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let id = None;
        for attr in attributes {
            if attr.path.is_ident("read") {}
        }
        Self { id }
    }
}

pub fn impl_enum(ast: &syn::DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let attrs = EnumAttributes::parse(&ast.attrs);

    let fields: Vec<_> = data
        .variants
        .iter()
        .map(|v| {
            if !matches!(v.fields, Fields::Unit) {
                panic!("Can only be used on c-style enums.");
            }
            if v.discriminant.is_none() {
                panic!(
                    "All enum variants must have an explicit value. E.g. Error = 0, Success = 1"
                );
            }
            let expr = match v.discriminant.as_ref().map(|(_, expr)| expr).unwrap() {
                syn::Expr::Cast(c) => c.expr.as_ref(),
                expr => expr,
            };
            let attributes = EnumFieldAttributes::parse(&v.attrs);
            (&v.ident, expr, attributes)
        })
        .collect();

    let read_type = attrs.read_type;
    let read_write = {
        let (idents, values): (Vec<_>, Vec<_>) = fields
            .iter()
            .map(|(idents, values, _)| (idents, values))
            .clone()
            .into_iter()
            .unzip();

        impl_read_write(&name, &idents, &values, &read_type)
    };
    let id_impl = {
        let (idents, ids): (Vec<_>, Vec<_>) = fields
            .iter()
            .filter_map(|(idents, _, attrs)| {
                if let Some(id) = &attrs.id {
                    Some((idents, id))
                } else {
                    None
                }
            })
            .clone()
            .into_iter()
            .unzip();
        if !idents.is_empty() {
            impl_identifiable(&name, &idents, &ids)
        } else {
            TokenStream::new()
        }
    };
    let x = quote! {
        #read_write
        #id_impl
    };
    //eprintln!("{}", x);
    x
}

fn impl_read_write(
    name: &syn::Ident,
    idents: &[&syn::Ident],
    values: &[&syn::Expr],
    read_type: &syn::Ident,
) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl<const PV: usize> ::mooshroom_core::io::MooshroomReadable<PV> for #name {
            fn read(reader: &mut impl ::std::io::Read) -> ::mooshroom_core::error::Result<Self> {
                let value = <#read_type as ::mooshroom_core::io::MooshroomReadable<PV>>::read(reader)?.into();
                Ok(
                    match value {
                        #( #values => #name::#idents, )*
                        i => return Err(::mooshroom_core::error::MooshroomError::InvalidEnumVariant(i.into()))
                    }
                )
            }
        }

        #[automatically_derived]
        impl<const PV: usize> ::mooshroom_core::io::MooshroomWritable<PV> for #name {
            fn write(&self, writer: &mut impl ::std::io::Write) -> ::mooshroom_core::error::Result<()> {
                match self {
                    #( #name::#idents => <#read_type as ::mooshroom_core::io::MooshroomWritable<PV>>::write(&((#values).into()), writer)?, )*
                };
                Ok(())
            }
        }
    }
}

fn impl_identifiable(
    name: &syn::Ident,
    idents: &[&syn::Ident],
    ids: &[&syn::LitStr],
) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl ::mooshroom_core::data::MooshroomIdentifiable for #name {
            type Type = &'static str;
            fn from_id(id: Self::Type) -> ::mooshroom_core::error::Result<Self>{
                match self {
                    #( #name::#idents => Ok(#ids), )*
                    _ => Err(::mooshroom_core::error::MooshroomError::InvalidId(id.into()))
                }
            }
            fn to_id(&self) -> ::mooshroom_core::error::Result<Self::Type> {
                match self {
                    #( #ids => Ok(#name::#idents), )*
                    _ => Err(::mooshroom_core::error::MooshroomError::NoId)
                }
            }
        }
    }
}
