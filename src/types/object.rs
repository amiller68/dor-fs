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
/// - metadata: the metadata of the file (this should be an IPLD Map)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Object {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    cid: Cid,
    metadata: Value,
}

impl Object {
    pub fn new(cid: Cid) -> Self {
        Self {
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

    // pub fn created_at(&self) -> &DateTime<Utc> {
    //     &self.created_at
    // }
    // pub fn updated_at(&self) -> &DateTime<Utc> {
    //     &self.updated_at
    // }

    pub fn cid(&self) -> &Cid {
        &self.cid
    }
    // pub fn metadata(&self) -> &Value {
    //     &self.metadata
    // }

    // pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
    //     self.updated_at = updated_at;
    // }

    // pub fn set_metadata(&mut self, metadata: Value) {
    //     self.metadata = metadata;
    // }
}
