use std::collections::{BTreeMap, HashMap};

use crate::Result;

pub trait FromMapping: Sized {
    fn from_map(mapping: impl Mapping) -> Result<Self>;
}

pub trait Mapping {
    fn get(&self, key: &str) -> Option<&String>;
}

impl Mapping for HashMap<String, String> {
    fn get(&self, key: &str) -> Option<&String> {
        self.get(key)
    }
}

impl Mapping for BTreeMap<String, String> {
    fn get(&self, key: &str) -> Option<&String> {
        self.get(key)
    }
}
