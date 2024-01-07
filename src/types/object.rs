#[allow(dead_code)]
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
/// A set of Objects within DorFS
pub struct ObjectSet(BTreeMap<PathBuf, Object>);

impl Deref for ObjectSet {
    type Target = BTreeMap<PathBuf, Object>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ObjectSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// TODO: this needs to be more idiomatically IPLD (should utilize links and IPLD types)
/// A single Object with DorFS metadata
/// - created_at: the time the file was added to the DorFS
/// - updated_at: the time the file was last updated
/// - cid: the cid of the file (this should be an IPLD link)
/// - metadata: Map from a schema name to a JSON object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Object {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    cid: Cid,
    metadata: BTreeMap<String, Value>,
}

impl Object {
    pub fn new(cid: Cid) -> Self {
        Self {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            cid,
            metadata: BTreeMap::new(),
        }
    }

    pub fn update(&mut self, cid: Cid) {
        self.cid = cid;
        self.updated_at = Utc::now();
    }

    // pub fn created_at(&self) -> &DateTime<Utc> {
    //     &self.created_at
    // }

    // pub fn updated_at(&self) -> &DateTime<Utc> {
    //     &self.updated_at
    // }

    pub fn cid(&self) -> &Cid {
        &self.cid
    }

    // pub fn metadata_entry(&self, schema_name: &String) -> Option<&Value> {
    //     self.metadata.get(schema_name)
    // }

    pub fn tag(&mut self, schema_name: &String, value: &Value) {
        self.metadata.insert(schema_name.clone(), value.clone());
        self.updated_at = Utc::now();
    }
}
