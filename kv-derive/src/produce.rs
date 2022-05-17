use proc_macro2::TokenStream;
use quote::quote;

use crate::Field;

pub(crate) fn generate_produce_fields(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_produce_field).collect()
}

fn generate_produce_field(field: &Field) -> TokenStream {
    let ident = field
        .ident
        .as_ref()
        .expect("unnamed fields are not implemented");
    let key = field.get_key();

    quote! {
        ::kv_derive_impl::Producer::produce(&self.#ident, &mut pairs, #key);
    }
}
