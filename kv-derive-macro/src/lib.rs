use proc_macro::TokenStream;
use quote::quote;

use crate::field::Field;
use crate::opts::{get_fields, parse_opts, MacroOpts};

mod field;
mod opts;

/// Derives [`kv_derive::into_vec::IntoVec`].
#[proc_macro_derive(IntoVec, attributes(kv))]
pub fn derive_into_vec(input: TokenStream) -> TokenStream {
    let opts: MacroOpts = parse_opts(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let field_producers = get_fields(opts.data)
        .into_iter()
        .map(generate_field_producer);

    let tokens = quote! {
        impl #generics ::kv_derive::into_vec::IntoVec for #ident #generics {
            fn into_iter(self) -> Box<dyn Iterator<Item = (String, String)>> {
                Box::new(
                    (std::iter::empty())
                    #(#field_producers)*
                )
            }
        }
    };
    tokens.into()
}

fn generate_field_producer(field: Field) -> proc_macro2::TokenStream {
    let ident = field.get_ident();
    let key = field.get_key();
    let value = if let Some(into_repr_with) = &field.into_repr_with {
        quote! { (#into_repr_with)(self.#ident) }
    } else {
        quote! { (self.#ident) }
    };

    let producer = if let Some(flatten) = &field.flatten {
        if let Some(prefix) = &flatten.prefix {
            quote! { (::kv_derive::producer::PrefixedFlatteningProducer(#value, #prefix)) }
        } else {
            quote! { (::kv_derive::producer::FlatteningProducer(#value)) }
        }
    } else {
        quote! { (#value) }
    };

    quote! {
        .chain(::kv_derive::producer::Producer::produce(#producer, #key))
    }
}

/// Derives [`kv_derive::from_iter::FromIter`].
#[proc_macro_derive(FromIter, attributes(kv))]
pub fn derive_from_iter(input: TokenStream) -> TokenStream {
    let opts: MacroOpts = parse_opts(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let fields = get_fields(opts.data);

    let field_consumers = fields.iter().map(generate_match_field_consumer);

    let field_defaults = fields.iter().map(|field| {
        let ident = field.get_ident();
        let ty = &field.ty;
        let default_opts = field
            .default
            .as_ref()
            .expect("`FromIter` requires `#[kv(default(…))]` on each field");
        if let Some(value) = &default_opts.value {
            quote! { #ident: (#value), }
        } else {
            quote! { #ident: (<#ty as std::default::Default>::default()), }
        }
    });

    let tokens = quote! {
        impl #generics ::kv_derive::from_iter::FromIter for #ident #generics {
            fn from_iter<'a>(iter: impl std::iter::IntoIterator<Item = (&'a str, &'a str)>) -> ::kv_derive::result::Result<Self> {
                let mut this = Self {
                    #(#field_defaults)*
                };
                for (key, value) in iter.into_iter() {
                    match key {
                        #(#field_consumers)*
                        _ => {}
                    }
                }
                Ok(this)
            }
        }
    };
    tokens.into()
}

fn generate_match_field_consumer(field: &Field) -> proc_macro2::TokenStream {
    assert!(
        field.flatten.is_none(),
        "restoring a flattened field from an iterable is not implemented",
    );

    let ident = field.get_ident();
    let key = field.get_key();
    let ty = &field.ty;

    let mut value = quote! { ::kv_derive::from_repr::FromRepr::from_repr(value)? };
    if let Some(from_repr_with) = &field.from_repr_with {
        value = quote! { ((#from_repr_with)(#value)?) };
    }

    quote! {
        #key => { <#ty as ::kv_derive::consumer::Consumer>::consume(&mut this.#ident, #value); }
    }
}

/// Derives [`kv_derive::from_mapping::FromMapping`].
#[proc_macro_derive(FromMapping, attributes(kv))]
pub fn derive_from_mapping(input: TokenStream) -> TokenStream {
    let opts: MacroOpts = parse_opts(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let mapped_fields = get_fields(opts.data).into_iter().map(generate_mapped_field);

    let tokens = quote! {
        impl #generics ::kv_derive::from_mapping::FromMapping for #ident #generics {
            fn from_mapping(mapping: impl Mapping) -> ::kv_derive::result::Result<Self> {
                Ok(Self {
                    #(#mapped_fields)*
                })
            }
        }
    };
    tokens.into()
}

fn generate_mapped_field(field: Field) -> proc_macro2::TokenStream {
    let ident = field.get_ident();
    assert!(
        field.flatten.is_none() || field.default.is_none(),
        "cannot use `flatten` and `default` at the same time for `{}`",
        ident,
    );

    if let Some(flatten) = &field.flatten {
        let mapping = if let Some(prefix) = &flatten.prefix {
            quote! { (::kv_derive::from_mapping::PrefixedMapping(mapping, #prefix)) }
        } else {
            quote! { (mapping) }
        };
        quote! { #ident: ::kv_derive::from_mapping::FromMapping::from_mapping(#mapping)?, }
    } else {
        let key = field.get_key();

        let missing_handler = if let Some(default) = &field.default {
            if let Some(value) = &default.value {
                quote! { Ok(#value) }
            } else {
                quote! { Ok(std::default::Default::default()) }
            }
        } else {
            quote! { Err(::kv_derive::error::Error::MissingKey(#key)) }
        };

        let mut value = quote! {
            ::kv_derive::consumer::Consumer::init(
                ::kv_derive::from_repr::FromRepr::from_repr(value)?,
            )
        };
        if let Some(from_repr_with) = &field.from_repr_with {
            value = quote! { ((#from_repr_with)(#value)?) };
        }

        quote! {
            #ident: mapping
                .get_value(#key)
                .map_or_else(
                    || #missing_handler,
                    |value| ::kv_derive::result::Result::Ok(#value),
                )?,
        }
    }
}
