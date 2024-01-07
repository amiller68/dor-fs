use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

use super::object::{Object, ObjectSet};
use super::schema::Schema;

// TODO: use IPLD for this
/// A DorFS
/// - object_set: a set of Objects within the DorFS
/// - previous_root: the cid of the previous root of the DorFS
/// - version: the version of the DorFS
/// - schemas: the schemas of the DorFS -- map of unique schema names to schema definitions, which are just JSON objects
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct DorStore {
    object_set: ObjectSet,
    previous_root: Cid,
    version: Version,
    schema: Schema,
}

impl DorStore {
    pub fn set_previous_root(&mut self, cid: Cid) {
        self.previous_root = cid;
    }

    pub fn objects(&self) -> &ObjectSet {
        &self.object_set
    }

    pub fn insert_object(&mut self, path: PathBuf, object: Object) {
        self.object_set.insert(path, object);
    }

    pub fn remove_object(&mut self, path: &PathBuf) {
        self.object_set.remove(path);
    }

    pub fn get_object_mut(&mut self, path: &PathBuf) -> Option<&mut Object> {
        self.object_set.get_mut(path)
    }

    pub fn update_object(&mut self, path: &PathBuf, cid: Cid) -> &Object {
        let object = self.get_object_mut(path).unwrap();
        object.update(cid);
        object
    }

    pub fn tag_object(&mut self, path: &PathBuf, schema_name: &String, value: &Value) -> &Object {
        let object = self.get_object_mut(path).unwrap();
        object.tag(schema_name, value);
        object
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn insert_schema_entry(&mut self, name: &String, fields: &Vec<String>) {
        self.schema.insert(name.clone(), fields.clone());
    }

    pub fn remove_schema_entry(&mut self, name: &String) {
        self.schema.remove(name);
    }

    pub fn get_schema_entry(&mut self, name: &String) -> Option<&Vec<String>> {
        self.schema.get(name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub build_profile: String,
    pub build_features: String,
    pub repo_version: String,
    pub version: String,
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    pub fn new() -> Self {
        Self {
            build_profile: env!("BUILD_PROFILE").to_string(),
            build_features: env!("BUILD_FEATURES").to_string(),
            repo_version: env!("REPO_VERSION").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
