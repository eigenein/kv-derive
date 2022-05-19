use proc_macro::TokenStream;
use quote::quote;

use crate::field::Field;
use crate::opts::{get_fields, MacroOpts};

mod field;
mod opts;

/// Derives [`kv_derive_impl::IntoVec`].
#[proc_macro_derive(IntoVec, attributes(kv))]
pub fn into_vec(input: TokenStream) -> TokenStream {
    let opts = MacroOpts::parse(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let field_producers = get_fields(opts.data)
        .into_iter()
        .map(generate_field_producer);

    let tokens = quote! {
        impl #generics ::kv_derive_impl::into_vec::IntoVec for #ident #generics {
            fn into_iter(self) -> Box<dyn Iterator<Item = (String, String)>> {
                Box::new(
                    std::iter::empty()
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

    let producer = if let Some(flatten) = &field.flatten {
        if let Some(prefix) = &flatten.prefix {
            quote! { ::kv_derive_impl::producer::PrefixedFlatteningProducer(self.#ident, #prefix) }
        } else {
            quote! { ::kv_derive_impl::producer::FlatteningProducer(self.#ident) }
        }
    } else {
        quote! { self.#ident }
    };

    quote! {
        .chain(::kv_derive_impl::producer::Producer::produce(#producer, #key))
    }
}

/// Derives [`kv_derive_impl::FromIter`].
#[proc_macro_derive(FromIter, attributes(kv))]
pub fn from_iter(input: TokenStream) -> TokenStream {
    let opts = MacroOpts::parse(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let field_consumers = get_fields(opts.data)
        .into_iter()
        .inspect(|field| {
            assert!(
                field.flatten.is_none(),
                "restoring a flattened structure from an iterable is not possible",
            )
        })
        .map(generate_match_field_consumer);

    let tokens = quote! {
        impl #generics ::kv_derive_impl::from_iter::FromIter for #ident #generics {
            fn from_iter<'a>(iter: impl std::iter::IntoIterator<Item = (&'a str, &'a str)>) -> ::kv_derive_impl::result::Result<Self>
            where
                Self: std::default::Default,
            {
                let mut this = Self::default();
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

fn generate_match_field_consumer(field: Field) -> proc_macro2::TokenStream {
    let ident = field.get_ident();
    assert!(
        field.flatten.is_none() || field.default.is_none(),
        "don't know how to build a default flattened field `{}`",
        ident,
    );
    let key = field.get_key();
    quote! {
        #key => { kv_derive_impl::consumer::Consumer::consume(&mut this.#ident, value)?; }
    }
}

/// Derives [`kv_derive_impl::FromMapping`].
#[proc_macro_derive(FromMapping, attributes(kv))]
pub fn from_mapping(input: TokenStream) -> TokenStream {
    let opts = MacroOpts::parse(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let mapped_fields = get_fields(opts.data).into_iter().map(generate_mapped_field);

    let tokens = quote! {
        impl #generics ::kv_derive_impl::from_mapping::FromMapping for #ident #generics {
            fn from_mapping(mapping: impl Mapping) -> ::kv_derive_impl::result::Result<Self> {
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
        "don't know how to build a default flattened field `{}`",
        ident,
    );
    let ty = &field.ty;
    let key = field.get_key();

    let missing_handler = if let Some(default) = &field.default {
        if let Some(value) = &default.value {
            quote! { Ok(#value) }
        } else {
            quote! { Ok(<#ty>::default()) }
        }
    } else {
        quote! { Err(::kv_derive_impl::error::Error::MissingKey(#key)) }
    };

    // TODO: support `flatten`.
    // TODO: support `Vec<_>`.

    quote! {
        #ident: mapping
            .get(#key)
            .map_or_else(
                || #missing_handler,
                ::kv_derive_impl::from_repr::FromRepr::from_repr,
            )?,
    }
}
