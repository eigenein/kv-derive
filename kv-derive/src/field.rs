use anyhow::{bail, Result};
use darling::FromField;
use syn::Ident;

#[derive(FromField)]
#[darling(attributes(kv), forward_attrs(allow, doc, cfg))]
pub(crate) struct Field {
    pub ident: Option<Ident>,

    /// Use the specified key instead of the field identifier.
    #[darling(default, rename = "rename")]
    pub custom_key: Option<String>,

    #[darling(default, rename = "optional")]
    pub is_optional: bool,
}

impl Field {
    pub fn get_key(&self) -> Result<String> {
        if let Some(custom_key) = &self.custom_key {
            return Ok(custom_key.clone());
        }
        if let Some(ident) = &self.ident {
            return Ok(format!("{}", ident));
        }
        bail!("the field is missing the identifier, did you mean to use `kv(rename = ...)`?")
    }
}
