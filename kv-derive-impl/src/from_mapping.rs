use std::collections::{BTreeMap, HashMap};

use crate::result::Result;

/// Constructs the structure from a mapping such as [`std::collections::HashMap`].
pub trait FromMapping: Sized {
    fn from_mapping(mapping: impl Mapping) -> Result<Self>;
}

/// Abstracts concrete map types so that [`FromMapping`] could accept any of the implementors.
pub trait Mapping {
    fn get(&self, key: &str) -> Option<&str>;
}

/// Wraps another mapping so that the values are got from the prefixed keys.
pub struct PrefixedMapping<T>(pub T, pub &'static str);

impl<T: Mapping> Mapping for PrefixedMapping<T> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(&format!("{}{}", self.1, key))
    }
}

impl Mapping for HashMap<String, String> {
    fn get(&self, key: &str) -> Option<&str> {
        HashMap::<String, String>::get(self, key).map(String::as_str)
    }
}

impl Mapping for &HashMap<String, String> {
    fn get(&self, key: &str) -> Option<&str> {
        HashMap::<String, String>::get(self, key).map(String::as_str)
    }
}

impl Mapping for BTreeMap<String, String> {
    fn get(&self, key: &str) -> Option<&str> {
        BTreeMap::<String, String>::get(self, key).map(String::as_str)
    }
}

impl Mapping for &BTreeMap<String, String> {
    fn get(&self, key: &str) -> Option<&str> {
        BTreeMap::<String, String>::get(self, key).map(String::as_str)
    }
}

impl Mapping for HashMap<&str, &str> {
    fn get(&self, key: &str) -> Option<&str> {
        HashMap::<&str, &str>::get(self, key).copied()
    }
}

impl Mapping for &HashMap<&str, &str> {
    fn get(&self, key: &str) -> Option<&str> {
        HashMap::<&str, &str>::get(self, key).copied()
    }
}

impl Mapping for BTreeMap<&str, &str> {
    fn get(&self, key: &str) -> Option<&str> {
        BTreeMap::<&str, &str>::get(self, key).copied()
    }
}

impl Mapping for &BTreeMap<&str, &str> {
    fn get(&self, key: &str) -> Option<&str> {
        BTreeMap::<&str, &str>::get(self, key).copied()
    }
}
