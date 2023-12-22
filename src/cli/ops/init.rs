use crate::cli::config::{Config, ConfigError};

pub fn init(config: &Config) -> Result<(), InitError> {
    config.init()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
}