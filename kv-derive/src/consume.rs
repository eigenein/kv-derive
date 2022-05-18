use proc_macro2::TokenStream;
use quote::quote;

use crate::Field;

pub(crate) fn generate_field_consumers(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(generate_field_consumer).collect()
}

fn generate_field_consumer(field: &Field) -> TokenStream {
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
