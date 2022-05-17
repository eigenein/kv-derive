use darling::FromField;
use syn::Ident;

#[derive(FromField)]
#[darling(attributes(kv), forward_attrs(allow, doc, cfg))]
pub(crate) struct Field {
    pub ident: Option<Ident>,

    /// Use the specified key instead of the field identifier.
    #[darling(default, rename = "rename")]
    pub custom_key: Option<String>,
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
}
