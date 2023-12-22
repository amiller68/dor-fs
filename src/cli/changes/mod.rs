use cid::Cid;
use serde::{Deserialize, Serialize};

use crate::types::DorStore;

mod log;

pub use log::{ChangeType, DisplayableLog, Log};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChangeLog {
    /// Alias of the managing device
    manager_alias: String,
    /// The log of changes
    log: Log,
    /// The versions of the DorFS currently staged
    versions: Vec<(Cid, DorStore)>,
}

impl ChangeLog {
    pub fn new(manager_alias: String, dor_store: &DorStore, root_cid: &Cid) -> Self {
        let mut log = Log::new();
        for (path, object) in dor_store.objects().iter() {
            log.insert(path.clone(), (object.cid().clone(), ChangeType::Base));
        }
        Self {
            manager_alias,
            log,
            versions: vec![(root_cid.clone(), dor_store.clone())],
        }
    }

    pub fn wipe(&mut self, dor_store: &DorStore, root_cid: &Cid) {
        let mut log = Log::new();
        for (path, object) in dor_store.objects().iter() {
            log.insert(path.clone(), (object.cid().clone(), ChangeType::Base));
        }
        self.log = log;
        self.versions = vec![(root_cid.clone(), dor_store.clone())];
    }

    pub fn update(&mut self, log: &Log, dor_store: &DorStore, root_cid: &Cid) {
        self.log = log.clone();
        self.versions.push((root_cid.clone(), dor_store.clone()));
    }

    pub fn manager_alias(&self) -> &String {
        &self.manager_alias
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn displayable(&self) -> DisplayableLog {
        DisplayableLog(self.log.clone())
    }

    pub fn first_version(&self) -> Option<&(Cid, DorStore)> {
        self.versions.first()
    }

    pub fn last_version(&self) -> Option<&(Cid, DorStore)> {
        self.versions.last()
    }
}
