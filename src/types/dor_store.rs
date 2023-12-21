use std::path::PathBuf;

use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::object::{Object, ObjectSet};

// TODO: use IPLD for this
/// A DorFS
/// - object_set: a set of Objects within the DorFS
/// - previous_root: the cid of the previous root of the DorFS
/// - version: the version of the DorFS
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct DorStore {
    object_set: ObjectSet,
    previous_root: Cid,
    version: Version,
}

impl DorStore {
    pub fn new() -> Self {
        Self {
            object_set: ObjectSet::default(),
            previous_root: Cid::default(),
            version: Version::new(),
        }
    }

    pub fn previous_root(&self) -> &Cid {
        &self.previous_root
    }

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

    pub fn update_object(&mut self, path: &PathBuf, cid: Cid, metadata: Option<Value>) -> &Object {
        let object = self.get_object_mut(path).unwrap();
        object.update(cid, metadata);
        object
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
