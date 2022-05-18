use proc_macro2::TokenStream;
use quote::quote;

use crate::field::Field;

pub(crate) fn generate_field_producers(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_field_producer).collect()
}

fn generate_field_producer(field: &Field) -> TokenStream {
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
