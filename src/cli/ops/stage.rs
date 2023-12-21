use std::fs::File;
use std::path::PathBuf;

use cid::Cid;

use super::change_log::ChangeType;
use super::utils::{load_change_log, load_dor_store, save_change_log, save_dor_store};

use crate::cli::config::{Config, ConfigError};
use crate::ipfs::{add_file_request, IpfsApi, IpfsClient, IpfsClientError, IpfsError};
use crate::types::Object;

/// Add a file to the local ipfs node using its path
async fn add_file(path: &PathBuf) -> Result<Cid, StageError> {
    let local_ipfs_client = IpfsClient::default();
    let file = File::open(path)?;
    let add_response = local_ipfs_client
        .add_with_options(file, add_file_request())
        .await?;
    let cid = Cid::try_from(add_response.hash)?;
    Ok(cid)
}

pub async fn stage(_config: &Config, working_dir: PathBuf) -> Result<(), StageError> {
    let mut dorfs = load_dor_store(working_dir.clone())?;
    let change_log = load_change_log(working_dir.clone())?;

    let mut change_log_update = change_log.clone();
    let change_log_iter = change_log.iter();
    // Iterate over the ChangeLog
    for (path, (cid, diff_type)) in change_log_iter {
        // Skip unchanged files -- mark changed files as base
        if diff_type == &ChangeType::Base {
            continue;
        }
        change_log_update.insert(path.clone(), (cid.clone(), ChangeType::Base));

        let working_path = working_dir.join(path);
        if diff_type == &ChangeType::Added || diff_type == &ChangeType::Modified {
            // Add the file to the local ipfs node
            let added_cid = add_file(&working_path).await?;
            // Make sure the cid matches the one in the change_log
            if added_cid != *cid {
                return Err(StageError::CidMismatch(added_cid, cid.clone()));
            }
            // Insert the file into the DorStore
            if diff_type == &ChangeType::Added {
                let object = Object::new(added_cid.clone());
                dorfs.insert_object(path.clone(), object.clone());
            } else if diff_type == &ChangeType::Modified {
                let mut d = dorfs.clone();
                let object = d.update_object(path, added_cid.clone(), None);
                dorfs.insert_object(path.clone(), object.clone());
            }
        }

        // If the file is a file, we just remove it from the DorStore
        // It won't be visible, but should be within the Fs History
        if diff_type == &ChangeType::Removed {
            dorfs.remove_object(path);
        }
    }

    save_change_log(working_dir.clone(), &change_log_update)?;
    save_dor_store(working_dir, &dorfs)?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum StageError {
    #[error("found changed dir in change_log")]
    DirInChangeLog,
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("encountered mismatched cid: {0} != {1}")]
    CidMismatch(Cid, Cid),
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("fs-tree error: {0}")]
    FsTree(#[from] fs_tree::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse diff: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("could not strip prefix: {0}")]
    PathPrefix(#[from] std::path::StripPrefixError),
    #[error("ipfs error: {0}")]
    Ipfs(#[from] IpfsError),
    #[error("ipfs client error: {0}")]
    IpfsClient(#[from] IpfsClientError),
}
