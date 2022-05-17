use proc_macro::TokenStream;
use quote::quote;

use crate::consume::generate_consume_fields;
use crate::field::Field;
use crate::opts::{get_fields, MacroOpts};
use crate::produce::generate_produce_fields;

mod consume;
mod field;
mod opts;
mod produce;

/// Generates `fn to_vec(&self) -> Vec<&'static str, String> {...}`.
#[proc_macro_derive(ToVec, attributes(kv))]
pub fn to_vec(input: TokenStream) -> TokenStream {
    let opts = MacroOpts::parse(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let produce_fields = generate_produce_fields(&get_fields(opts.data));

    let tokens = quote! {
        impl #generics #ident {
            pub fn to_vec(&self) -> std::vec::Vec<(&'static str, String)> {
                let mut pairs = std::vec::Vec::new();
                #(#produce_fields)*
                pairs
            }
        }
    };
    tokens.into()
}

/// Generates `fn from_iter(iter: IntoIterator<...>) -> anyhow::Result<Self> {...}`.
#[proc_macro_derive(FromIter, attributes(kv))]
pub fn from_iter(input: TokenStream) -> TokenStream {
    let opts = MacroOpts::parse(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let consume_fields = generate_consume_fields(&get_fields(opts.data));

    let tokens = quote! {
        impl #generics #ident {
            pub fn from_iter<'a>(iter: impl std::iter::IntoIterator<Item = (&'a str, &'a str)>) -> ::kv_derive_impl::Result<Self>
            where
                Self: std::default::Default,
            {
                let mut this = Self::default();
                for (key, value) in iter.into_iter() {
                    match key {
                        #(#consume_fields)*
                        _ => {}
                    }
                }
                Ok(this)
            }
        }
    };
    tokens.into()
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../../README.md"));
}
