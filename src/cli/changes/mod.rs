use cid::Cid;
use serde::{Deserialize, Serialize};

use crate::types::Manifest;

mod log;

pub use log::{ChangeType, DisplayableLog, Log};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChangeLog {
    /// Alias of the managing device
    manager_alias: String,
    /// The log of changes
    log: Log,
    /// The versions of the DorFS currently staged
    versions: Vec<(Cid, Manifest)>,
}

impl ChangeLog {
    pub fn new(manager_alias: String, manifest: &Manifest, root_cid: &Cid) -> Self {
        let mut log = Log::new();
        for (path, object) in manifest.objects().iter() {
            log.insert(path.clone(), (*object.cid(), ChangeType::Base));
        }
        Self {
            manager_alias,
            log,
            versions: vec![(*root_cid, manifest.clone())],
        }
    }

    pub fn wipe(&mut self, manifest: &Manifest, root_cid: &Cid) {
        let mut log = Log::new();
        for (path, object) in manifest.objects().iter() {
            log.insert(path.clone(), (*object.cid(), ChangeType::Base));
        }
        self.log = log;
        self.versions = vec![(*root_cid, manifest.clone())];
    }

    pub fn update(&mut self, log: &Log, manifest: &Manifest, root_cid: &Cid) {
        self.log = log.clone();
        self.versions.push((*root_cid, manifest.clone()));
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

    pub fn first_version(&self) -> Option<&(Cid, Manifest)> {
        self.versions.first()
    }

    pub fn last_version(&self) -> Option<&(Cid, Manifest)> {
        self.versions.last()
    }
}
