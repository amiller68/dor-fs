use std::path::PathBuf;

use crate::cli::config::{Config, ConfigError};

use super::utils::load_root_cid;

pub fn health(_config: &Config, working_dir: PathBuf) -> Result<(), HealthError> {
    let root_cid = load_root_cid(working_dir.clone())?;
    println!("root_cid: {}", root_cid);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    #[error("critical failure")]
    CriticalFailure,
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
}
