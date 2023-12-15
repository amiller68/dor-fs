use crate::cli::config::Config;

pub fn health(_config: &Config) -> Result<(), HealthError> {
    Err(HealthError::CriticalFailure)
}

#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    #[error("critical failure")]
    CriticalFailure,
}
