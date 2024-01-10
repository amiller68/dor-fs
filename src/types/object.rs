use chrono::{DateTime, Utc};
use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// TODO: this needs to be more idiomatically IPLD (should utilize links and IPLD types)
/// A single Object with DorFS metadata
/// - created_at: the time the file was added to the DorFS
/// - updated_at: the time the file was last updated
/// - cid: the cid of the file (this should be an IPLD link)
/// - metadata: This can be any piece of Json metadata you want 
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
            metadata: Value::Null
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

    pub fn set_metdata(&mut self, value: Value) {
        self.metadata = value;
        self.updated_at = Utc::now();
    }
}
