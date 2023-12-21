use std::io::Write;
use std::path::PathBuf;

use cid::Cid;
use fs_tree::FsTree;

use crate::cli::config::{Config, ConfigError, DEFAULT_LOCAL_DOT_CHANGELOG};
use crate::ipfs::{hash_file_request, IpfsApi, IpfsClient, IpfsClientError};

use super::change_log::{ChangeLog, ChangeType};
use super::utils::{dot_dir, working_dot_dir};

const DIR_HASH_LABEL: &str = "DIRERCTORY";

// TODO: prolly doing alot of unecessary shit here
/// Construct a diff from the local dot directory
pub async fn diff(_config: &Config, working_dir: PathBuf) -> Result<(), DiffError> {
    // Check if the local config exists in the working directory
    let working_local_dot_path = working_dot_dir(working_dir.clone())?;

    // Read the local dot directory for a change_log -- this is the base
    let local_dot_change_log_path = working_local_dot_path.join(DEFAULT_LOCAL_DOT_CHANGELOG);

    // The files we know about from our last pull
    let mut base = match std::fs::read_to_string(local_dot_change_log_path.clone()) {
        Ok(s) => {
            let change_log: ChangeLog = serde_json::from_str(&s)?;
            change_log
        }
        Err(_) => {
            tracing::info!("No change_log found");
            // TODO: this should generate an error
            ChangeLog::new()
        }
    };

    let mut update = base.clone();

    // Insert the root directory hash into the change_log for comparison
    // This should always just get matched out and removed
    base.insert(PathBuf::from(""), (Cid::default(), ChangeType::Base));

    // Read the working directory structure into a fs-tree
    let next = next_fs_tree(working_dir.clone())?;

    // Iterate over the path-sorted change_log and the fs-tree in order to diff
    let mut base_iter = base.iter();
    let mut next_iter = next.iter();

    let mut next_next = next_iter.next();
    let mut base_next = base_iter.next();

    loop {
        match (next_next.clone(), base_next) {
            // If these are both something we got some work to do
            (Some((_next_tree, next_path)), Some((base_path, (base_hash, base_type)))) => {
                // For each object, assuming we stay aligned on a sorted list of paths:
                // If the base comes before then this file was removed
                // strip off the base object and log the removal
                if base_path < &next_path {
                    let working_base_path = working_dir.clone().join(base_path);
                    if !working_base_path.is_dir() {
                        match base_type {
                            ChangeType::Added => {
                                update.remove(base_path);
                            }
                            _ => {
                                update.insert(
                                    base_path.clone(),
                                    (Cid::default(), ChangeType::Removed),
                                );
                            }
                        }
                    }
                    base_next = base_iter.next();
                    continue;
                }

                // If next comes before base then the file was added
                // strip off the next object and log the addition
                if &next_path < base_path {
                    let working_next_path = working_dir.clone().join(next_path.clone());
                    if !working_next_path.is_dir() {
                        let hash = file_cid(working_next_path).await?;
                        update.insert(next_path.clone(), (hash.clone(), ChangeType::Added));
                    }
                    next_next = next_iter.next();
                    continue;
                }

                // If they are equal then we are good. Move on to the next objects
                if &next_path == base_path {
                    // These are either both files or both directories
                    // If they are both files then we need to compare hashes
                    let working_next_path = working_dir.clone().join(next_path.clone());
                    if !working_next_path.is_dir() {
                        // If the hashes are different then the file was modified
                        // strip off the next object and log the modification
                        let next_hash = file_cid(working_next_path).await?;
                        if base_hash != &next_hash {
                            match base_type {
                                ChangeType::Added => {
                                    update.insert(
                                        base_path.clone(),
                                        (next_hash.clone(), ChangeType::Added),
                                    );
                                }
                                _ => {
                                    update.insert(
                                        base_path.clone(),
                                        (next_hash.clone(), ChangeType::Modified),
                                    );
                                }
                            }
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
                    let hash = file_cid(working_next_path).await?;
                    update.insert(next_path.clone(), (hash.clone(), ChangeType::Added));
                }
                next_next = next_iter.next();
                continue;
            }

            // There's more new files than old, this file was added
            (None, Some((base_path, (_base_hash, base_type)))) => {
                let working_base_path = working_dir.clone().join(base_path);
                if !working_base_path.is_dir() {
                    match base_type {
                        ChangeType::Added => {
                            update.remove(base_path);
                        }
                        _ => {
                            update.insert(base_path.clone(), (Cid::default(), ChangeType::Removed));
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

    // Write the change_log to disk
    let mut file = std::fs::File::create(local_dot_change_log_path)?;
    let change_log = serde_json::to_string(&update)?;
    file.write_all(change_log.as_bytes())?;

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

pub async fn file_cid(path: PathBuf) -> Result<Cid, DiffError> {
    let client = IpfsClient::default();
    let file = std::fs::File::open(path)?;
    let add_response = client.add_with_options(file, hash_file_request()).await?;
    let cid = Cid::try_from(add_response.hash)?;
    Ok(cid)
}

#[derive(Debug, thiserror::Error)]
pub enum DiffError {
    #[error("could not read change_log: {0}")]
    ReadChanges(#[from] serde_json::Error),
    #[error("is not a directory")]
    NotADirectory,
    #[error("fs-tree error: {0}")]
    FsTree(#[from] fs_tree::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid config: {0}")]
    Config(#[from] ConfigError),
    #[error("ipfs error: {0}")]
    Ipfs(#[from] IpfsClientError),
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
}
