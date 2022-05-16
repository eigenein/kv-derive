use darling::ast::Data;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use to_vec::ToVecOpts;

mod field;
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
        let key = format!("{}", ident);
        quote! {
            pairs.push((#key, self.#ident.to_string()));
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
