use proc_macro2::TokenStream;
use quote::quote;

use crate::field::{Field, FlattenOpts};

pub(crate) fn generate_produce_fields(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_produce_field).collect()
}

fn generate_produce_field(field: &Field) -> TokenStream {
    let ident = field.get_ident();
    let key = field.get_key();

    let this = match &field.flatten {
        Some(FlattenOpts { prefix: None }) => {
            quote! { ::kv_derive_impl::producer::FlatteningProducer(self.#ident) }
        }
        Some(FlattenOpts {
            prefix: Some(prefix),
        }) => {
            quote! { ::kv_derive_impl::producer::PrefixedFlatteningProducer(self.#ident, #prefix) }
        }
        _ => quote! { self.#ident },
    };

    quote! {
        .chain(::kv_derive_impl::producer::Producer::produce(#this, #key))
    }
}
