use darling::ast::Data;
use darling::util::Ignored;
use darling::FromDeriveInput;
use syn::{Generics, Ident};

use crate::field::Field;

#[derive(FromDeriveInput)]
#[darling(supports(struct_named, struct_unit), forward_attrs(allow, doc, cfg))]
pub(crate) struct FromSliceOpts {
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data<Ignored, Field>,
}
