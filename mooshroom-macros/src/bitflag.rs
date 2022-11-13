use syn::{Attribute, Data, DataStruct};
use quote::quote;

struct MooshroomBitflagAttributes {
    value_type: syn::Ident,
}

impl MooshroomBitflagAttributes {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut value_type: Option<syn::Ident> = None;

        for attr in attributes {
            if attr.path.is_ident("value_type") {
                value_type = attr.parse_args().unwrap()
            }
        }
        Self {
            value_type: value_type.expect("Must have #[value_type(T)]"),
        }
    }
}
struct MooshroomBitflagFieldsAttrs {
    pub mask: syn::LitInt
}

impl MooshroomBitflagFieldsAttrs {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut mask = None;
        for attr in attributes {
            if attr.path.is_ident("mask") {
                mask = Some(attr.parse_args().unwrap())
            }
        }
        Self{
            mask: mask.expect("each field have #[mask(value)]")
        }
    }
}

pub fn impl_mooshroom_bitflag(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        Data::Struct(s) => impl_mooshroom_bitfield_struct(ast, s),
        _ => unimplemented!("impl_mooshroom_bitflag"),
    }
}

fn impl_mooshroom_bitfield_struct(
    ast: &syn::DeriveInput,
    data: &DataStruct,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let MooshroomBitflagAttributes { value_type } = MooshroomBitflagAttributes::parse(&ast.attrs);

    let (fields, masks) : (Vec<_>, Vec<_>) = data.fields.iter().map(|f| {
        let field_attr = MooshroomBitflagFieldsAttrs::parse(&f.attrs);
        (&f.ident, field_attr.mask)
    }).unzip();

    quote! {
        #[automatically_derived]
        impl ::mooshroom_core::data::MooshroomBitFlag for #name {
            type Type = #value_type;
            fn from_value(t: Self::Type) -> Self {
                Self{
                    #( #fields: (t&#masks) == #masks,  )*
                }
            }
            fn to_value(&self) -> Self::Type {
                let mut value = 0.into();
                #( if self.#fields { value |=  #masks; }  )*
                value
            }
        }
    }
}