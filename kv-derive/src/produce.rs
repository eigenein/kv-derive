use proc_macro2::TokenStream;
use quote::quote;

use crate::field::Field;

pub(crate) fn generate_produce_fields(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_produce_field).collect()
}

fn generate_produce_field(field: &Field) -> TokenStream {
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
