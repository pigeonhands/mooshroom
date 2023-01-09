use quote::quote;
use syn::{Attribute, Data, DataEnum, Fields};

enum FieldIdType {
    Single(syn::LitInt),
    Range(syn::ExprRange),
}
#[derive(Default)]
struct CollectionFieldAttributes {
    id: Option<FieldIdType>,
}

impl CollectionFieldAttributes {
    pub fn parse(attributes: &[Attribute]) -> Self {
        let mut field_attrs = Self::default();
        for attr in attributes {
            if attr.path.is_ident("id") {
                match field_attrs.id {
                    Some(_) => panic!("can not have both #[id()] and #[id_range()]"),
                    None => {
                        field_attrs.id = Some(FieldIdType::Single(
                            attr.parse_args().expect("id must be a integer"),
                        ))
                    }
                }
            } else if attr.path.is_ident("id_range") {
                match field_attrs.id {
                    Some(_) => panic!("can not have both #[id()] and #[id_range()]"),
                    None => {
                        field_attrs.id = Some(FieldIdType::Range(
                            attr.parse_args()
                                .expect("#[id_range()] must be a range (0..5)"),
                        ))
                    }
                }
            }
        }
        field_attrs
    }
}

pub fn impl_collection(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        Data::Enum(s) => impl_collection_enum(ast, s),
        _ => unimplemented!("impl_mooshroom_packet"),
    }
}

fn impl_collection_enum(ast: &syn::DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let fields: Vec<_> = data
        .variants
        .iter()
        .map(|f| {
            if let Fields::Unnamed(un) = &f.fields {
                (&f.ident, un, f)
            } else {
                panic!("All enum fields must have a value. Use a unit for a no-read value.")
            }
        })
        .map(|(name, data, variant)| {
            let f = data.unnamed.first().unwrap();
            let ty = &f.ty;
            let attrs = CollectionFieldAttributes::parse(&variant.attrs);
            (name, ty, attrs)
        })
        .collect();

    let read_selector = fields.iter().map(|(name, ty, attrs)|{
            if let Some(id_type) = &attrs.id {
                match id_type {
                    FieldIdType::Single(id) => {
                        quote! {
                            ::mooshroom_core::varint::VarInt(#id) => Ok(Self::#name(<#ty as ::mooshroom_core::io::MooshroomReadable<PV>>::read(reader)?)),
                        }
                    },
                    FieldIdType::Range(range) => {
                        quote! {
                            ::mooshroom_core::varint::VarInt(#range) => Ok(Self::#name(<#ty as ::mooshroom_core::data::MooshroomCollection<PV>>::read_one_of(id, reader)?)),
                        }
                    }
                }
            }else{
                quote! {
                    <#ty as ::mooshroom_core::io::MooshroomPacket<PV>>::PACKET_ID => Ok(Self::#name(<#ty as ::mooshroom_core::io::MooshroomReadable<PV>>::read(reader)?)),
                }
            }
        });
    let write_selector = fields.iter().map(|(name, ty, attrs)|{
            if let Some(FieldIdType::Range(_)) = &attrs.id {
                quote! {
                    Self::#name(value) => <#ty as ::mooshroom_core::data::MooshroomCollection<PV>>::write_one_of(value, writer)?,
                }
            }else{
                quote! {
                    Self::#name(value) => <#ty as ::mooshroom_core::io::MooshroomWritable<PV>>::write(value, writer)?,
                }
            }
        });

    let ids_selectors = fields.iter().map(|(name, ty, attrs)|{
        if let Some(id_type) = &attrs.id {
            match id_type {
                FieldIdType::Single(id) => {
                    quote! {
                        Self::#name(_) => ::mooshroom_core::varint::VarInt(#id),
                    }
                },
                FieldIdType::Range(_) => {
                    quote! {
                        Self::#name(range_field) => <#ty as ::mooshroom_core::data::MooshroomCollection<PV>>::variant_id(&range_field),
                    }
                }
            }
        }else{
            quote! {
                Self::#name(_) => <#ty as ::mooshroom_core::io::MooshroomPacket<PV>>::PACKET_ID,
            }
        }
    });
    let n = quote! {
        #[automatically_derived]
        impl<const PV: ::mooshroom_core::io::Protocal> ::mooshroom_core::data::MooshroomCollection<PV> for #name {
            fn read_one_of(id: ::mooshroom_core::varint::VarInt, reader: &mut impl ::std::io::Read) -> ::mooshroom_core::error::Result<Self>{
                match id {
                    #( #read_selector ) *
                    i => Err(::mooshroom_core::error::MooshroomError::NotInCollection(i.0))
                }
            }
            fn write_one_of(&self,writer: &mut impl ::std::io::Write) -> ::mooshroom_core::error::Result<()> {
                match self {
                    #( #write_selector ) *
                }
                Ok(())
            }
            fn variant_id(&self) -> ::mooshroom_core::varint::VarInt {
                match self {
                    #( #ids_selectors ) *
                }
            }
        }
    };
    //eprintln!("{:#}", n);
    n
}
