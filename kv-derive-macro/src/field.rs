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

    /// Flattening options.
    #[darling(default)]
    pub flatten: Option<FlattenOpts>,

    /// Default value.
    #[darling(default)]
    pub default: Option<DefaultOpts>,

    #[darling(default, rename = "optional")]
    pub is_optional: bool,

    #[darling(default, rename = "collection")]
    pub is_collection: bool,

    #[darling(default)]
    pub from_repr_with: Option<Expr>,

    #[darling(default)]
    pub into_repr_with: Option<Expr>,
}

#[derive(Default, FromMeta)]
#[darling(default)]
pub(crate) struct FlattenOpts {
    /// Prefix all the flattened fields with the specified prefix.
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

    /// Builds a field consumer based on the attributes.
    pub fn consumer(&self) -> TokenStream {
        let mut consumer = quote! {
            ::kv_derive::consumer::ScalarConsumer(std::marker::PhantomData)
        };
        if self.is_optional {
            consumer = quote! { ::kv_derive::consumer::OptionConsumer(#consumer) };
        }
        if self.is_collection {
            consumer = quote! { ::kv_derive::consumer::CollectionConsumer(#consumer) };
        }
        consumer
    }

    /// Wraps the value with an expression of `from_repr_with`, if specified.
    pub fn wrap_from_repr_with(&self, value: TokenStream) -> TokenStream {
        if let Some(from_repr_with) = &self.from_repr_with {
            quote! { ((#from_repr_with)(#value)?) }
        } else {
            value
        }
    }
}
