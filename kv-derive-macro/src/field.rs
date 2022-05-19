use darling::{FromField, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Ident, Type};

#[derive(FromField)]
#[darling(attributes(kv), forward_attrs(allow, doc, cfg))]
pub(crate) struct Field {
    pub ident: Option<Ident>,

    pub ty: Type,

    /// Use the specified key instead of the field identifier.
    #[darling(default, rename = "rename")]
    pub custom_key: Option<String>,

    #[darling(default)]
    pub flatten: Option<FlattenOpts>,

    #[darling(default)]
    pub default: Option<DefaultOpts>,

    #[darling(default)]
    pub via: Option<Type>,
}

#[derive(Default, FromMeta)]
#[darling(default)]
pub(crate) struct FlattenOpts {
    pub prefix: Option<String>,
}

#[derive(Default, FromMeta)]
#[darling(default)]
pub(crate) struct DefaultOpts {
    pub value: Option<Expr>,
}

impl Field {
    /// Gets the target key, either the field name or a custom key.
    pub fn get_key(&self) -> String {
        if let Some(custom_key) = &self.custom_key {
            return custom_key.clone();
        }
        if let Some(ident) = &self.ident {
            return format!("{}", ident);
        }
        panic!("the field is missing the identifier, did you mean to use `[kv(rename = …)]` or `[kv(flatten(…))]`?")
    }

    /// Unwraps the field identifier. Placeholder to support unnamed fields in future.
    pub fn get_ident(&self) -> &Ident {
        self.ident
            .as_ref()
            .expect("unnamed fields are not implemented")
    }

    pub fn representation_type(&self) -> TokenStream {
        let ty = &self.ty;
        if let Some(via) = &self.via {
            quote! { <#via as ::kv_derive_impl::from_repr::FromRepr> }
        } else {
            quote! { <#ty as ::kv_derive_impl::consumer::Consumer>::Repr }
        }
    }
}
