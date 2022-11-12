use quote::quote;
use syn::{Attribute, DataEnum, Fields};

struct EnumValueAttributes {
    read_type: syn::Ident,
}

impl EnumValueAttributes {
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

pub fn impl_enum(ast: &syn::DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let attrs = EnumValueAttributes::parse(&ast.attrs);

    let (idents, values): (Vec<_>, Vec<_>) = data
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
            (&v.ident, expr)
        })
        .unzip();

    let read_type = attrs.read_type;
    let x = quote! {
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
    };
    x
}
