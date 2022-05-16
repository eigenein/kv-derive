use darling::ast::Data;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

use crate::from_slice::FromSliceOpts;
use crate::to_vec::ToVecOpts;

mod field;
mod from_slice;
mod to_vec;

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(ToVec, attributes(kv))]
pub fn to_vec(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse the input");
    let opts = ToVecOpts::from_derive_input(&ast).expect("failed to parse the struct options");

    let ident = opts.ident;
    let generics = opts.generics;

    let fields = match opts.data {
        Data::Enum(_) => unimplemented!("enums are not implemented"),
        Data::Struct(fields) => fields.fields,
    };
    let push_fields = fields.into_iter().map(|field| {
        let ident = field.ident.expect("unnamed fields are not implemented");
        let ty = field.ty;
        let key = format!("{}", ident);
        quote! {
            let value = <#ty as std::string::ToString>::to_string(&self.#ident);
            pairs.push((#key, value));
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

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(FromSlice, attributes(kv))]
pub fn from_slice(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse the input");
    let opts = FromSliceOpts::from_derive_input(&ast).expect("failed to parse the struct options");

    let ident = opts.ident;
    let generics = opts.generics;

    let tokens = quote! {
        impl #generics #ident {
            pub fn from_slice<'a>(slice: impl std::iter::IntoIterator<Item = &'a (&'a str, &'a str)>) -> ::anyhow::Result<Self>
            where
                Self: std::default::Default,
            {
                Ok(Self::default())
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

    external_doc_test!(include_str!("../README.md"));
}
