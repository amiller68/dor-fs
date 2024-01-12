use std::fs::File;
use std::path::PathBuf;

use cid::Cid;

use super::diff::{diff, DiffError};

use crate::cli::changes::ChangeType;
use crate::cli::config::{Config, ConfigError};
use crate::device::{Device, DeviceError};
use crate::types::Object;

/// Stage a file against the local ipfs node
pub async fn stage_file(device: &Device, file_path: &PathBuf) -> Result<Cid, StageError> {
    let file = File::open(file_path)?;
    // Write the dor store against the local instance
    let cid = device.write_ipfs_data(file, false).await?;
    Ok(cid)
}

pub async fn stage(config: &Config) -> Result<(), StageError> {
    let device = config.device()?;
    let working_dir = config.working_dir().clone();
    let updates = diff(config).await?;
    let mut change_log = config.change_log()?;
    let base_manifest = config.base()?;
    let (last_root_cid, last_manifest) = change_log.last_version().unwrap().clone();
    let mut update_manifest = base_manifest.clone();

    let change_log_iter = updates.iter();
    // Iterate over the ChangeLog -- play updates against the base ... probably better to do this
    for (path, (cid, diff_type)) in change_log_iter {
        // Skip unchanged files -- mark changed files as base
        if diff_type == &ChangeType::Base {
            continue;
        }
        // updates.insert(path.clone(), (cid.clone(), ChangeType::Staged));

        let working_path = working_dir.join(path);
        if diff_type == &ChangeType::Added || diff_type == &ChangeType::Modified {
            // Add the file to the local ipfs node
            let added_cid = stage_file(&device, &working_path).await?;
            // Make sure the cid matches the one in the change_log
            if added_cid != *cid {
                return Err(StageError::CidMismatch(added_cid, *cid));
            }
            // Insert the file into the Manifest
            if diff_type == &ChangeType::Added {
                let object = Object::new(added_cid);
                update_manifest.insert_object(path, &object);
            } else if diff_type == &ChangeType::Modified {
                let object = update_manifest.get_object_mut(path).unwrap();
                object.update(added_cid);
            }
        }

        // If the file is a file, we just remove it from the Manifest
        // It won't be visible, but should be within the Fs History
        if diff_type == &ChangeType::Removed {
            update_manifest.remove_object(path);
        }
    }

    if update_manifest == last_manifest {
        tracing::info!("no changes to stage");
        return Ok(());
    }

    update_manifest.set_previous_root(last_root_cid);

    // Hash the dor store against the remote
    let update_root_cid = device.hash_manifest(&update_manifest, false).await?;

    change_log.update(&updates, &update_manifest, &update_root_cid);

    config.set_change_log(change_log)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum StageError {
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
    #[error("diff error: {0}")]
    Diff(#[from] DiffError),
    #[error("device error: {0}")]
    Device(#[from] DeviceError),
}
