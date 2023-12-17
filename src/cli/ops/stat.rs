use std::path::PathBuf;

use super::change_log::DisplayableChangeLog;
use super::utils::load_change_log;

use crate::cli::config::{Config, ConfigError};

pub fn stat(_config: &Config, working_dir: PathBuf) -> Result<DisplayableChangeLog, StatError> {
    let change_log = load_change_log(working_dir)?;
    Ok(DisplayableChangeLog(change_log))
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
