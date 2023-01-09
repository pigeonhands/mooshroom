use quote::quote;
use syn::Attribute;
struct MooshroomUpdatableAttrs {
    pub update_using: syn::Ident,
}

impl MooshroomUpdatableAttrs {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut update_using = None;
        for attr in attributes {
            if attr.path.is_ident("update_using") {
                update_using = Some(attr.parse_args().unwrap())
            }
        }
        Self {
            update_using: update_using.expect("must have #[update_using(EnumType)]"),
        }
    }
}

enum UpdateFieldFrom {
    Direct(syn::Path),
    Extends(syn::Path),
}
struct MooshroomUpdatableFieldsAttrs {
    pub from: UpdateFieldFrom,
}

impl MooshroomUpdatableFieldsAttrs {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut from = None;
        for attr in attributes {
            if attr.path.is_ident("from") {
                from.map(|_| panic!("cannot have both #[from] and #[extends]"));
                from = Some(UpdateFieldFrom::Direct(
                    attr.parse_args().expect("Must be EnumType::Variant"),
                ))
            } else if attr.path.is_ident("extends") {
                from.map(|_| panic!("cannot have both #[from] and #[extends]"));
                from = Some(UpdateFieldFrom::Extends(
                    attr.parse_args().expect("Must be EnumType::Variant"),
                ))
            }
        }
        Self {
            from: from.expect(
                "each field have #[from(EnumType::variant)] or #[extends(EnumType::variant)].",
            ),
        }
    }
}

pub fn impl_mooshroom_updatable(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
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
    let MooshroomUpdatableAttrs { update_using } = MooshroomUpdatableAttrs::parse(&ast.attrs);

    let field_info: Vec<_> = data
        .fields
        .iter()
        .map(|f| {
            let field_attrs = MooshroomUpdatableFieldsAttrs::parse(&f.attrs);
            (&f.ident, &f.ty, field_attrs.from)
        })
        .collect();

    let update_fields : Vec<_> = field_info.iter().map(|(ident, ty, from)| {
        match from {
            UpdateFieldFrom::Direct(d) => quote!{
                #update_using::#d(value) => self.#ident = value
            },
            UpdateFieldFrom::Extends(e) => quote!{
                #update_using::#e(value) => <#ty as ::mooshroom_core::data::MooshroomUpdatable>::update(&mut self.#ident, value)
            }
        }
    }).collect();

    let x = quote! {
        #[automatically_derived]
        impl ::mooshroom_core::data::MooshroomUpdatable for #name {
            type Type = #update_using;
            fn update(&mut self, value: Self::Type){
                match value {
                    #( #update_fields, ) *
                    _ => {}
                };
            }
        }
    };
    //eprintln!("{:#}", x);
    x
}
