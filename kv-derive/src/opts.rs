use darling::ast::Data;
use darling::util::Ignored;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{Generics, Ident};

use crate::field::Field;

/// The macro options.
#[derive(FromDeriveInput)]
#[darling(supports(struct_named, struct_unit), forward_attrs(allow, doc, cfg))]
pub(crate) struct MacroOpts {
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data<Ignored, Field>,
}

impl MacroOpts {
    pub(crate) fn parse(input: TokenStream) -> Self {
        let ast = syn::parse(input).expect("failed to parse the input");
        Self::from_derive_input(&ast).expect("failed to parse the macro options")
    }
}
