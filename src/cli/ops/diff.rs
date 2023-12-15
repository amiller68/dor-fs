use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

use fs_tree::FsTree;
use serde::{Deserialize, Serialize};

use crate::cli::config::{
    dot_dir, working_dot_dir, Config, ConfigError, DEFAULT_LOCAL_DOT_CHANGELOG,
};

const DIR_HASH_LABEL: &str = "DIRERCTORY";

// TODO: this type isn't quite right
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DiffType {
    Base,
    Added,
    Modified,
    Removed,
}

impl std::fmt::Display for DiffType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Base => "\x1b[0;32mBase\x1b[0m",
            Self::Added => "\x1b[0;32mAdded\x1b[0m",
            Self::Modified => "\x1b[0;33mModified\x1b[0m",
            Self::Removed => "\x1b[0;31mRemoved\x1b[0m",
        };
        write!(f, "{}", s)
    }
}

/// Tracks what files are in the local clone and their hashes
type Changelog = BTreeMap<PathBuf, (String, DiffType)>;

pub struct DisaplyableChangelog(pub Changelog);

impl std::fmt::Display for DisaplyableChangelog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (path, (_hash, diff_type)) in self.0.iter() {
            if diff_type == &DiffType::Base {
                continue;
            }
            s.push_str(&format!("{}: {}\n", path.to_str().unwrap(), diff_type));
        }
        write!(f, "{}", s)
    }
}

// TODO: prolly doing alot of unecessary shit here
/// Construct a diff from the local dot directory
pub fn diff(_config: &Config, working_dir: PathBuf) -> Result<(), DiffError> {
    // Check if the local config exists in the working directory
    let working_local_dot_path = working_dot_dir(working_dir.clone())?;

    // Read the local dot directory for a changelog -- this is the base
    let local_dot_changelog_path = working_local_dot_path.join(DEFAULT_LOCAL_DOT_CHANGELOG);

    // The files we know about from our last pull
    let mut base = match std::fs::read_to_string(local_dot_changelog_path.clone()) {
        Ok(s) => {
            let changelog: Changelog = serde_json::from_str(&s)?;
            changelog
        }
        Err(_) => {
            tracing::info!("No changelog found");
            // TODO: this should generate an error
            BTreeMap::new()
        }
    };

    let mut update = base.clone();

    // Insert the root directory hash into the changelog for comparison
    // This should always just get matched out and removed
    base.insert(
        PathBuf::from(""),
        (DIR_HASH_LABEL.to_string(), DiffType::Base),
    );

    // Read the working directory structure into a fs-tree
    let next = next_fs_tree(working_dir.clone())?;

    // Iterate over the path-sorted changelog and the fs-tree in order to diff
    let mut base_iter = base.iter();
    let mut next_iter = next.iter();

    let mut next_next = next_iter.next();
    let mut base_next = base_iter.next();


    loop {
        match (next_next.clone(), base_next) {
            // If these are both something we got some work to do
            (Some((_next_tree, next_path)), Some((base_path, (base_hash, base_type)))) => {
                // For each item, assuming we stay aligned on a sorted list of paths:
                // If the base comes before then this file was removed
                // strip off the base item and log the removal
                if base_path < &next_path {
                    // if !working_dir.join(base_path).is_dir() {
                    let working_base_path = working_dir.clone().join(base_path);
                    if !working_base_path.is_dir() {
                        match base_type {
                            DiffType::Added => {
                                update.remove(base_path);
                            }
                            _ => {
                                update.insert(base_path.clone(), (blank_hash(), DiffType::Removed));
                            }
                        }
                    }
                    base_next = base_iter.next();
                    continue;
                }

                // If next comes before base then the file was added
                // strip off the next item and log the addition
                if &next_path < base_path {
                    let working_next_path = working_dir.clone().join(next_path.clone());
                    if !working_next_path.is_dir() {
                        let hash = blake3_hash_file(working_next_path)?;
                        update.insert(next_path.clone(), (hash.clone(), DiffType::Added));
                    }
                    next_next = next_iter.next();
                    continue;
                }

                // If they are equal then we are good. Move on to the next items
                if &next_path == base_path {
                    // These are either both files or both directories
                    // If they are both files then we need to compare hashes
                    let working_next_path = working_dir.clone().join(next_path.clone());
                    if !working_next_path.is_dir() {
                        // If the hashes are different then the file was modified
                        // strip off the next item and log the modification
                        let next_hash = blake3_hash_file(working_next_path)?;
                        if base_hash != &next_hash {
                            update
                                .insert(base_path.clone(), (next_hash.clone(), DiffType::Modified));
                        }
                    }

                    next_next = next_iter.next();
                    base_next = base_iter.next();
                    continue;
                }
            }

            // Theres more old file than new, this file was removed
            (Some((_next_tree, next_path)), None) => {
                let working_next_path = working_dir.clone().join(next_path.clone());
                if !working_next_path.is_dir() {
                    let hash = blake3_hash_file(working_next_path)?;
                    update.insert(next_path.clone(), (hash.clone(), DiffType::Added));
                }
                next_next = next_iter.next();
                continue;
            }

            // There's more new files than old, this file was added
            (None, Some((base_path, (_base_hash, base_type)))) => {
                let working_base_path = working_dir.clone().join(base_path);
                if !working_base_path.is_dir() {
                    match base_type {
                        DiffType::Added => {
                            update.remove(base_path);
                        }
                        _ => {
                            update.insert(base_path.clone(), (blank_hash(), DiffType::Removed));
                        }
                    }
                }
                base_next = base_iter.next();
                continue;
            }
            (None, None) => {
                // We are done
                break;
            }
        }
    }

    // Write the changelog to disk
    let mut file = std::fs::File::create(local_dot_changelog_path)?;
    let changelog = serde_json::to_string(&update)?;
    file.write_all(changelog.as_bytes())?;

    Ok(())
}

/// Get the next fs-tree from the working directory           
fn next_fs_tree(working_dir_path: PathBuf) -> Result<FsTree, DiffError> {
    // Read Fs-tree at dir or pwd, stripping off the local dot directory
    let next = match fs_tree::FsTree::read_at(working_dir_path.to_str().unwrap())? {
        fs_tree::FsTree::Directory(mut d) => {
            let _res = &d.remove_entry(&dot_dir());
            fs_tree::FsTree::Directory(d)
        }
        _ => {
            return Err(DiffError::NotADirectory);
        }
    };
    Ok(next)
}

// TODO: is this using threads?
fn blake3_hash_file(path: PathBuf) -> Result<String, DiffError> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(hash.to_hex().to_string())
}

fn blank_hash() -> String {
    blake3::Hash::from([0; 32]).to_hex().to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum DiffError {
    #[error("missing dot path")]
    MissingDotPath(PathBuf),
    #[error("could not read changelog: {0}")]
    ReadChanges(#[from] serde_json::Error),
    #[error("is not a directory")]
    NotADirectory,
    #[error("fs-tree error: {0}")]
    FsTree(#[from] fs_tree::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid config: {0}")]
    Config(#[from] ConfigError),
}
