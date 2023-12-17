use std::path::PathBuf;

use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::item::{Item, ItemSet};

// TODO: use IPLD for this
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DorFs {
    unix_fs_root: Cid,
    item_set: ItemSet,
    version: String,
}

impl DorFs {
    pub fn new() -> Self {
        Self {
            unix_fs_root: Cid::default(),
            item_set: ItemSet::default(),
            version: env!("REPO_VERSION").to_string(),
        }
    }

    pub fn unix_fs_root(&self) -> &Cid {
        &self.unix_fs_root
    }

    pub fn set_unix_fs_root(&mut self, cid: Cid) {
        self.unix_fs_root = cid;
    }

    pub fn item_set(&self) -> &ItemSet {
        &self.item_set
    }

    pub fn insert_item(&mut self, path: PathBuf, item: Item) {
        self.item_set.insert(path, item);
    }

    pub fn remove_item(&mut self, path: &PathBuf) {
        self.item_set.remove(path);
    }

    pub fn get_item(&self, path: &PathBuf) -> Option<&Item> {
        self.item_set.get(path)
    }

    pub fn get_item_mut(&mut self, path: &PathBuf) -> Option<&mut Item> {
        self.item_set.get_mut(path)
    }

    pub fn update_item(&mut self, path: &PathBuf, cid: Cid, metadata: Option<Value>) -> &Item {
        let mut item = self.get_item_mut(path).unwrap();
        item.update(cid, metadata);
        item
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub build_profile: &'static str,
    pub version: &'static str,
}

impl Version {
    pub fn new() -> Self {
        Self {
            build_profile: env!("BUILD_PROFILE"),
            version: env!("REPO_VERSION"),
        }
    }
}
