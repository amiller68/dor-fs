use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// TODO: idiomatic schemas that support typing, validation, and types other than strings

/// Map of unique schema names to Json Metadata definitions
/// For now all this supports is an array of string fields
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Schema(BTreeMap<String, Vec<String>>);

impl Schema {
    pub fn insert(&mut self, name: String, fields: Vec<String>) {
        self.0.insert(name, fields);
    }

    pub fn remove(&mut self, name: &str) {
        self.0.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&Vec<String>> {
        self.0.get(name)
    }
}
