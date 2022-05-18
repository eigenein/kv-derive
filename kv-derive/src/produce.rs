use proc_macro2::TokenStream;
use quote::quote;

use crate::Field;

pub(crate) fn generate_produce_fields(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_produce_field).collect()
}

fn generate_produce_field(field: &Field) -> TokenStream {
    let ident = field.get_ident();
    let key = field.get_key();

    let this = if let Some(_flatten) = &field.flatten {
        quote! { ::kv_derive_impl::producer::FlatteningProducer(self.#ident) }
    } else {
        quote! { self.#ident }
    };

    quote! {
        .chain(::kv_derive_impl::producer::Producer::produce(#this, #key))
    }
}
