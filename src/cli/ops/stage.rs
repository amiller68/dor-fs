use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use cid::Cid;

use super::change_log::ChangeType;
use super::utils::{load_change_log, load_dor_fs, save_change_log, save_dor_fs};

use crate::cli::config::{Config, ConfigError, DEFAULT_MFS_ROOT};
use crate::ipfs::{IpfsApi, IpfsClient};
use crate::types::Item;

pub async fn stage(_config: &Config, working_dir: PathBuf) -> Result<(), StageError> {
    let mut dorfs = load_dor_fs(working_dir.clone())?;
    let change_log = load_change_log(working_dir.clone())?;
    let local_ipfs_client = IpfsClient::default();

    let mut change_log_update = change_log.clone();
    let change_log_iter = change_log.iter();
    // Iterate over the ChangeLog
    for (path, (hash, diff_type)) in change_log_iter {
        if diff_type == &ChangeType::Base {
            continue;
        }
        change_log_update.insert(path.clone(), (hash.clone(), ChangeType::Base));

        let working_path = working_dir.join(path);
        let (mfs_dir, file_name) = mfs_path(path)?;

        if diff_type == &ChangeType::Added {
            // Create the directory
            let mfs_dir_str = mfs_dir.to_str().unwrap();
            local_ipfs_client.files_mkdir(&mfs_dir_str, true).await?;
        }

        if diff_type == &ChangeType::Added || diff_type == &ChangeType::Modified {
            // Open the file
            let file = File::open(working_path)?;
            let mfs_path = mfs_dir.join(file_name.clone());
            let mfs_path_str = mfs_path.to_str().unwrap();
            local_ipfs_client
                .files_write(&mfs_path_str, true, true, file)
                .await?;
            let mfs_stat = local_ipfs_client.files_stat(&mfs_path_str).await?;
            let mfs_hash = mfs_stat.hash;
            let mfs_cid = Cid::from_str(&mfs_hash)?;

            if diff_type == &ChangeType::Added {
                let item = Item::new(path.clone(), mfs_cid);
                dorfs.insert_item(path.clone(), item.clone());
            } else if diff_type == &ChangeType::Modified {
                let mut d = dorfs.clone();
                let item = d.update_item(path, mfs_cid, None);
                dorfs.insert_item(path.clone(), item.clone());
            }
        }

        // If the file is a file, we need to remove it
        if diff_type == &ChangeType::Removed {
            // Remove the file
            let mfs_path = mfs_dir.join(file_name);
            let mfs_path_str = mfs_path.to_str().unwrap();
            local_ipfs_client.files_rm(&mfs_path_str, true).await?;
            dorfs.remove_item(path);
        }
    }

    save_change_log(working_dir.clone(), &change_log_update)?;
    save_dor_fs(working_dir, &dorfs)?;
    Ok(())
}

/// Prepends the MFS root to the path. Return (ipfs_path, file_name)
fn mfs_path(path: &PathBuf) -> Result<(PathBuf, PathBuf), StageError> {
    let file_name = path.file_name().ok_or(StageError::DirInChangeLog)?;
    let mut path = path.clone();
    path.pop();
    let ipfs_path = PathBuf::from(DEFAULT_MFS_ROOT);
    Ok((ipfs_path.join(path), file_name.into()))
}

#[derive(Debug, thiserror::Error)]
pub enum StageError {
    #[error("found changed dir in change_log")]
    DirInChangeLog,
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
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
    Ipfs(#[from] ipfs_api_backend_hyper::Error),
}
