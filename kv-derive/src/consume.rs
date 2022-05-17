use proc_macro2::TokenStream;
use quote::quote;

use crate::Field;

pub(crate) fn generate_consume_fields(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_consume_field).collect()
}

fn generate_consume_field(field: &Field) -> TokenStream {
    let ident = field
        .ident
        .as_ref()
        .expect("unnamed fields are not implemented");
    let key = field.get_key();

    quote! {
        #key => { kv_derive_impl::Consumer::consume(&mut this.#ident, value)?; }
    }
}
