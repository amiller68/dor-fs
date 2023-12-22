use cid::Cid;
use serde::{Deserialize, Serialize};

use crate::types::DorStore;

mod log;

pub use log::{DisplayableLog, Log};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChangeLog {
    log: Log,
    root_cid: Cid,
    versions: Vec<(Cid, DorStore)>,
}

impl ChangeLog {
    pub fn new() -> Self {
        Self {
            log: Log::new(),
            root_cid: Cid::default(),
            versions: Vec::new(),
        }
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn log_mut(&mut self) -> &mut Log {
        &mut self.log
    }

    pub fn displayable(&self) -> DisplayableLog {
        DisplayableLog(self.log.clone())
    }

    pub fn root_cid(&self) -> &Cid {
        &self.root_cid
    }

    pub fn versions(&self) -> &Vec<(Cid, DorStore)> {
        &self.versions
    }

    pub fn add_version(&mut self, cid: Cid, store: DorStore) {
        self.versions.push((cid, store));
    }

    pub fn set_root_cid(&mut self, cid: Cid) {
        self.root_cid = cid;
    }
}
