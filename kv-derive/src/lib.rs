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

/// Generates [`kv_derive_impl::IntoVec`].
#[proc_macro_derive(IntoVec, attributes(kv))]
pub fn into_vec(input: TokenStream) -> TokenStream {
    let opts = MacroOpts::parse(input);
    let ident = opts.ident;
    let generics = opts.generics;
    let produce_fields = generate_produce_fields(&get_fields(opts.data));

    let tokens = quote! {
        impl #generics ::kv_derive_impl::IntoVec for #ident {
            fn into_iter(self) -> Box<dyn Iterator<Item = (String, String)>> {
                Box::new(
                    std::iter::empty()
                    #(#produce_fields)*
                )
            }
        }
    };
    tokens.into()
}

/// Generates [`kv_derive_impl::FromIter`].
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
