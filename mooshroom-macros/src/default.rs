use quote::quote;
use syn::Attribute;

struct MooshroomDefaultFieldsAttrs {
    pub default: Option<syn::Expr>,
}

impl MooshroomDefaultFieldsAttrs {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut default = None;
        for attr in attributes {
            if attr.path.is_ident("default") {
                default = Some(attr.parse_args().unwrap())
            }
        }
        Self { default }
    }
}

pub fn impl_mooshroom_default_inline(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        syn::Data::Struct(s) => impl_mooshroom_packet_struct(ast, s),
        _ => unimplemented!("impl_mooshroom_updatable"),
    }
}

fn impl_mooshroom_packet_struct(
    ast: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let field_data: Vec<_> = data
        .fields
        .iter()
        .map(|f| {
            let field_attrs = MooshroomDefaultFieldsAttrs::parse(&f.attrs);
            (&f.ident, field_attrs.default)
        })
        .collect();

    let field_inits: Vec<_> = field_data
        .iter()
        .map(|(ident, default)| {
            if let Some(d) = default {
                quote! {
                    #ident: #d
                }
            } else {
                quote! {
                    #ident: Default::default()
                }
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl Default for #name {
            fn default() -> Self {
                Self {
                    #( #field_inits, )*
                }
            }
        }
    }
}
