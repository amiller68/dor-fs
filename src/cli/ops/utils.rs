use std::path::{Path, PathBuf};

use cid::Cid;

use super::change_log::ChangeLog;

use crate::cli::config::{
    ConfigError, DEFAULT_LOCAL_DOT_CHANGELOG, DEFAULT_LOCAL_DOT_DIR, DEFAULT_LOCAL_DOT_DORFS,
    DEFAULT_LOCAL_DOT_ROOTCID,
};
use crate::types::DorStore;

pub fn local_dot_dorfs_path() -> Result<PathBuf, ConfigError> {
    let local_dot_dorfs_path = dot_dir().join(DEFAULT_LOCAL_DOT_DORFS);
    Ok(local_dot_dorfs_path)
}

pub fn working_dot_dir(working_dir: PathBuf) -> Result<PathBuf, ConfigError> {
    // Check if the local config exists in the working directory
    let local_dot_path = local_dot_dir_with_base(&working_dir);
    // If not then this dir has not been cloned
    if !local_dot_path.exists() || !local_dot_path.is_dir() {
        return Err(ConfigError::MissingDotPath(local_dot_path));
    }
    Ok(local_dot_path)
}

/// Path to the local dot directory tracking changes to the local filesystem
pub fn dot_dir() -> PathBuf {
    PathBuf::from(DEFAULT_LOCAL_DOT_DIR)
}

/// Path to the local dot directory tracking changes to the local filesystem
/// relative to the working directory
pub fn local_dot_dir_with_base(working_dir_path: &Path) -> PathBuf {
    working_dir_path.join(DEFAULT_LOCAL_DOT_DIR)
}

/// Load the ChangeLog from the local dot directory
pub fn load_change_log(path: PathBuf) -> Result<ChangeLog, ConfigError> {
    let working_dot_dir_path = working_dot_dir(path)?;
    let change_log_path = working_dot_dir_path.join(DEFAULT_LOCAL_DOT_CHANGELOG);
    let change_log = match std::fs::read_to_string(change_log_path) {
        Ok(s) => serde_json::from_str(&s)?,
        Err(_) => {
            // TODO: this should generate an error
            tracing::info!("No change_log found");
            ChangeLog::new()
        }
    };
    Ok(change_log)
}

/// Save the ChangeLog to the local dot directory
pub fn save_change_log(path: PathBuf, change_log: &ChangeLog) -> Result<(), ConfigError> {
    let working_dot_dir_path = working_dot_dir(path)?;
    let change_log_path = working_dot_dir_path.join(DEFAULT_LOCAL_DOT_CHANGELOG);
    let change_log = serde_json::to_string_pretty(change_log)?;
    std::fs::write(change_log_path, change_log)?;
    Ok(())
}

/// Load the DorStore from the local dot directory
pub fn load_dor_store(path: PathBuf) -> Result<DorStore, ConfigError> {
    let working_dot_dir_path = working_dot_dir(path)?;
    let dor_store_path = working_dot_dir_path.join(DEFAULT_LOCAL_DOT_DORFS);
    let dor_store = match std::fs::read_to_string(dor_store_path) {
        Ok(s) => serde_json::from_str(&s)?,
        Err(_) => DorStore::new(),
    };
    Ok(dor_store)
}

/// Save the DorStore to the local dot directory
pub fn save_dor_store(path: PathBuf, dor_store: &DorStore) -> Result<(), ConfigError> {
    let working_dot_dir_path = working_dot_dir(path)?;
    let dor_store_path = working_dot_dir_path.join(DEFAULT_LOCAL_DOT_DORFS);
    let dor_store = serde_json::to_string_pretty(dor_store)?;
    std::fs::write(dor_store_path, dor_store)?;
    Ok(())
}

/// Load the root cid from the local dot directory
pub fn load_root_cid(path: PathBuf) -> Result<Cid, ConfigError> {
    let working_dot_dir_path = working_dot_dir(path)?;
    let root_cid_path = working_dot_dir_path.join(DEFAULT_LOCAL_DOT_ROOTCID);
    let root_cid = match std::fs::read_to_string(root_cid_path) {
        Ok(s) => serde_json::from_str(&s)?,
        Err(_) => {
            return Err(ConfigError::MissingRootCid);
        }
    };
    Ok(root_cid)
}

/// Save the root cid to the local dot directory
pub fn save_root_cid(path: PathBuf, root_cid: &Cid) -> Result<(), ConfigError> {
    let working_dot_dir_path = working_dot_dir(path)?;
    let root_cid_path = working_dot_dir_path.join(DEFAULT_LOCAL_DOT_ROOTCID);
    let root_cid = serde_json::to_string_pretty(root_cid)?;
    std::fs::write(root_cid_path, root_cid)?;
    Ok(())
}
