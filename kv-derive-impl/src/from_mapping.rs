use std::collections::{BTreeMap, HashMap};

use crate::result::Result;

/// Constructs the structure from a mapping such as [`std::collections::HashMap`].
pub trait FromMapping: Sized {
    fn from_mapping(mapping: impl Mapping) -> Result<Self>;
}

/// Abstracts concrete map types so that [`FromMapping`] could accept any of the implementors.
pub trait Mapping {
    fn get_value(&self, key: &str) -> Option<&str>;
}

/// Wraps another mapping so that the values are got from the prefixed keys.
pub struct PrefixedMapping<T>(pub T, pub &'static str);

impl<T: Mapping> Mapping for PrefixedMapping<T> {
    fn get_value(&self, key: &str) -> Option<&str> {
        self.0.get_value(&format!("{}{}", self.1, key))
    }
}

macro_rules! impl_mapping {
    ($type:ty) => {
        impl Mapping for $type {
            fn get_value(&self, key: &str) -> Option<&str> {
                self.get(key).map(AsRef::as_ref)
            }
        }
    };
}

impl_mapping!(HashMap<&str, &str>);
impl_mapping!(HashMap<String, &str>);
impl_mapping!(HashMap<&str, String>);
impl_mapping!(HashMap<String, String>);
impl_mapping!(&HashMap<&str, &str>);
impl_mapping!(&HashMap<String, &str>);
impl_mapping!(&HashMap<&str, String>);
impl_mapping!(&HashMap<String, String>);

impl_mapping!(BTreeMap<&str, &str>);
impl_mapping!(BTreeMap<String, &str>);
impl_mapping!(BTreeMap<&str, String>);
impl_mapping!(BTreeMap<String, String>);
impl_mapping!(&BTreeMap<&str, &str>);
impl_mapping!(&BTreeMap<String, &str>);
impl_mapping!(&BTreeMap<&str, String>);
impl_mapping!(&BTreeMap<String, String>);
