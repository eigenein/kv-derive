use darling::{FromField, FromMeta};
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
    pub fn get_key(&self) -> String {
        if let Some(custom_key) = &self.custom_key {
            return custom_key.clone();
        }
        if let Some(ident) = &self.ident {
            return format!("{}", ident);
        }
        panic!("the field is missing the identifier, did you mean to use `kv(rename = ...)`?")
    }

    pub fn get_ident(&self) -> &Ident {
        self.ident
            .as_ref()
            .expect("unnamed fields are not implemented")
    }
}
