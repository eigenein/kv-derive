use darling::ast::Data;
use darling::util::Ignored;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

use crate::field::Field;
use crate::from_iter::FromSliceOpts;
use crate::to_vec::ToVecOpts;

mod field;
mod from_iter;
mod to_vec;

/// Generates `fn to_vec(&self) -> Vec<&'static str, String> {...}`.
#[proc_macro_derive(ToVec, attributes(kv))]
pub fn to_vec(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse the input");
    let opts = ToVecOpts::from_derive_input(&ast).expect("failed to parse the struct options");

    let ident = opts.ident;
    let generics = opts.generics;

    let push_fields = get_fields(opts.data).into_iter().map(|field| {
        let ident = field.ident.expect("unnamed fields are not implemented");
        let ty = field.ty;
        let key = format!("{}", ident);
        quote! {
            pairs.push((#key, <#ty as std::string::ToString>::to_string(&self.#ident)));
        }
    });

    let tokens = quote! {
        impl #generics #ident {
            pub fn to_vec(&self) -> std::vec::Vec<(&'static str, String)> {
                let mut pairs = std::vec::Vec::new();
                #(#push_fields)*
                pairs
            }
        }
    };
    tokens.into()
}

/// Generates `fn from_iter(iter: IntoIterator<...>) -> anyhow::Result<Self> {...}`.
#[proc_macro_derive(FromIter, attributes(kv))]
pub fn from_iter(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse the input");
    let opts = FromSliceOpts::from_derive_input(&ast).expect("failed to parse the struct options");

    let ident = opts.ident;
    let generics = opts.generics;

    let match_and_set = get_fields(opts.data).into_iter().map(|field| {
        let ident = field.ident.expect("unnamed fields are not implemented");
        let ty = field.ty;
        let key = format!("{}", ident);
        quote! {
            #key => { this.#ident = <#ty as std::str::FromStr>::from_str(value)?; }
        }
    });

    let tokens = quote! {
        impl #generics #ident {
            pub fn from_iter<'a>(iter: impl std::iter::IntoIterator<Item = (&'a str, &'a str)>) -> ::anyhow::Result<Self>
            where
                Self: std::default::Default,
            {
                let mut this = Self::default();
                for (key, value) in iter.into_iter() {
                    match key {
                        #(#match_and_set)*
                        _ => {}
                    }
                }
                Ok(this)
            }
        }
    };
    tokens.into()
}

fn get_fields(data: Data<Ignored, Field>) -> Vec<Field> {
    match data {
        Data::Enum(_) => unimplemented!("enums are not implemented"),
        Data::Struct(fields) => fields.fields,
    }
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
