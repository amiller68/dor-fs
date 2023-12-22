use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use cid::Cid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Base,
    Added,
    Modified,
    Removed,
    Staged,
}

impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Base => "\x1b[0;32mBase\x1b[0m",
            Self::Added => "\x1b[0;32mAdded\x1b[0m",
            Self::Modified => "\x1b[0;33mModified\x1b[0m",
            Self::Removed => "\x1b[0;31mRemoved\x1b[0m",
            Self::Staged => "\x1b[0;34mStaged\x1b[0m",
        };
        write!(f, "{}", s)
    }
}

/// Tracks what files are in the local clone and their hashes
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Log(BTreeMap<PathBuf, (Cid, ChangeType)>);

impl Deref for Log {
    type Target = BTreeMap<PathBuf, (Cid, ChangeType)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Log {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Log {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

pub struct DisplayableLog(pub Log);

impl std::fmt::Display for DisplayableLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (path, (_hash, diff_type)) in self.0.iter() {
            if diff_type == &ChangeType::Base {
                continue;
            }
            s.push_str(&format!("{}: {}\n", path.to_str().unwrap(), diff_type));
        }
        write!(f, "{}", s)
    }
}
