use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ItemSet(BTreeMap<PathBuf, Item>);

impl Deref for ItemSet {
    type Target = BTreeMap<PathBuf, Item>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Item {
    name: String,
    path: PathBuf,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    cid: Cid,
    metadata: Value,
}

impl Item {
    pub fn new(path: PathBuf, cid: Cid) -> Self {
        let name = path
            .file_name()
            .unwrap_or_else(|| panic!("could not get file name for path: {:?}", path))
            .to_str()
            .unwrap_or_else(|| panic!("could not convert file name to str: {:?}", path))
            .to_string();

        Self {
            name,
            path,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            cid,
            metadata: Value::Null,
        }
    }

    pub fn update(&mut self, cid: Cid, maybe_metadata: Option<Value>) {
        self.cid = cid;
        if let Some(metadata) = maybe_metadata {
            self.metadata = metadata;
        }
        self.updated_at = Utc::now();
    }
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn cid(&self) -> &Cid {
        &self.cid
    }
    pub fn metadata(&self) -> &Value {
        &self.metadata
    }

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = updated_at;
    }

    pub fn set_metadata(&mut self, metadata: Value) {
        self.metadata = metadata;
    }
}
