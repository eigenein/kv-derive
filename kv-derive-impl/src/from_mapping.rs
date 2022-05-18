use std::collections::{BTreeMap, HashMap};

use crate::result::Result;

pub trait FromMapping: Sized {
    fn from_mapping(mapping: impl Mapping) -> Result<Self>;
}

pub trait Mapping {
    fn get(&self, key: &str) -> Option<&str>;
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
