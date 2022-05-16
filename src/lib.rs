use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Generics, Ident};

#[doc(hidden)]
#[allow(missing_docs)]
#[proc_macro_derive(ToKeyValues, attributes(kv))]
pub fn to_key_values(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let opts = ToKeyValuesOpts::from_derive_input(&ast).unwrap();
    let ident = opts.ident;
    let generics = opts.generics;

    let tokens = quote! {
        impl #generics #ident {
            pub fn to_key_values(&self) -> std::vec::Vec<(String, String)> {
                std::vec::Vec::new()
            }
        }
    };
    tokens.into()
}

#[derive(FromDeriveInput)]
#[darling(
    supports(struct_named, struct_unit),
    attributes(kv),
    forward_attrs(allow, doc, cfg)
)]
struct ToKeyValuesOpts {
    ident: Ident,
    generics: Generics,
}
