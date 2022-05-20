use darling::ast::Data;
use darling::util::Ignored;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{Generics, Ident};

use crate::field::Field;

/// Macro options for [`crate::into_vec::IntoVec`], [`crate::from_iter::FromIter`]
/// and [`crate::from_mapping::FromMapping`].
#[derive(FromDeriveInput)]
#[darling(supports(struct_named, struct_unit), forward_attrs(allow, doc, cfg))]
pub(crate) struct MacroOpts {
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data<Ignored, Field>,
}

pub(crate) fn parse_opts<T: FromDeriveInput>(input: TokenStream) -> T {
    let ast = syn::parse(input).expect("failed to parse the input");
    T::from_derive_input(&ast).expect("failed to parse the macro options")
}

pub(crate) fn get_fields(data: Data<Ignored, Field>) -> Vec<Field> {
    match data {
        Data::Enum(_) => unimplemented!("enums are not implemented"),
        Data::Struct(fields) => fields.fields,
    }
}
