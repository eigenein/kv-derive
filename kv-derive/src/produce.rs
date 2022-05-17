use quote::quote;

use crate::Field;

pub(crate) fn produce_fields(fields: &[Field]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|field| {
            let ident = field
                .ident
                .as_ref()
                .expect("unnamed fields are not implemented");
            let key = field.get_key();

            quote! {
                ::kv_derive_impl::Producer::produce(&self.#ident, &mut pairs, #key);
            }
        })
        .collect()
}
