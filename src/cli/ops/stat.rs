use std::collections::BTreeMap;
use std::path::PathBuf;

use super::DisaplyableChangelog;
use crate::cli::config::{working_dot_dir, Config, ConfigError, DEFAULT_LOCAL_DOT_CHANGELOG};

pub fn stat(_config: &Config, working_dir: PathBuf) -> Result<DisaplyableChangelog, StatError> {
    let working_dot_dir_path = working_dot_dir(working_dir.clone())?;
    let changelog =
        match std::fs::read_to_string(working_dot_dir_path.join(DEFAULT_LOCAL_DOT_CHANGELOG)) {
            Ok(s) => serde_json::from_str(&s)?,
            Err(_) => {
                tracing::info!("No changelog found");
                BTreeMap::new()
            }
        };
    Ok(DisaplyableChangelog(changelog))
}

#[derive(Debug, thiserror::Error)]
pub enum StatError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse diff: {0}")]
    Serde(#[from] serde_json::Error),
}
