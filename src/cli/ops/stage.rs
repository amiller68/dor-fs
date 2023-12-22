use cid::Cid;

use super::diff::{diff, DiffError};

use crate::cli::changes::ChangeType;
use crate::cli::config::{Config, ConfigError};
use crate::device::DeviceError;
use crate::types::Object;

pub async fn stage(config: &Config) -> Result<(), StageError> {
    let device = config.device()?;
    let working_dir = config.working_dir().clone();
    let updates = diff(config).await?;
    let mut change_log = config.change_log()?;
    let base_dor_store = config.base()?;
    let (last_root_cid, last_dor_store) = change_log.last_version().unwrap().clone();
    let mut update_dor_store = base_dor_store.clone();

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
            let added_cid = device.stage(&working_path).await?;
            // Make sure the cid matches the one in the change_log
            if added_cid != *cid {
                return Err(StageError::CidMismatch(added_cid, cid.clone()));
            }
            // Insert the file into the DorStore
            if diff_type == &ChangeType::Added {
                let object = Object::new(added_cid.clone());
                update_dor_store.insert_object(path.clone(), object.clone());
            } else if diff_type == &ChangeType::Modified {
                let mut d = update_dor_store.clone();
                let object = d.update_object(path, added_cid.clone(), None);
                update_dor_store.insert_object(path.clone(), object.clone());
            }
        }

        // If the file is a file, we just remove it from the DorStore
        // It won't be visible, but should be within the Fs History
        if diff_type == &ChangeType::Removed {
            update_dor_store.remove_object(path);
        }
    }

    if update_dor_store == last_dor_store {
        tracing::info!("no changes to stage");
        return Ok(());
    }

    update_dor_store.set_previous_root(last_root_cid);

    let update_root_cid = device.hash_dor_store(&update_dor_store).await?;

    change_log.update(&updates, &update_dor_store, &update_root_cid);

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
