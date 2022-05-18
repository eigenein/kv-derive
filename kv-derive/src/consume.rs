use proc_macro2::TokenStream;
use quote::quote;

use crate::Field;

pub(crate) fn generate_consume_fields(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_consume_field).collect()
}

fn generate_consume_field(field: &Field) -> TokenStream {
    let ident = field.get_ident();
    let key = field.get_key();

    let consumer = if field.flatten.is_some() {
        panic!("restoring a flattened structure is not possible");
    } else {
        quote! { this.#ident }
    };

    quote! {
        #key => { kv_derive_impl::consumer::Consumer::consume(&mut #consumer, value)?; }
    }
}
