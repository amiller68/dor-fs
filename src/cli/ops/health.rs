use std::path::PathBuf;

use crate::cli::config::{Config, ConfigError};
use crate::root_cid::{EthClient, EthClientError};

use super::utils::load_root_cid;


// TODO: check if all services are reachable, print out relevant config info in a pretty way
pub async fn health(_config: &Config, working_dir: PathBuf) -> Result<(), HealthError> {
    let eth_remote = match _config.eth_remote() {
        Some(eth_remote) => eth_remote,
        None => {
            tracing::info!("eth_remote not configured");
            return Ok(());
        }
    };

    let ipfs_remote = match _config.ipfs_remote() {
        Some(ipfs_remote) => ipfs_remote,
        None => {
            tracing::info!("ipfs_remote not configured");
            return Ok(());
        }
    };

    let eth_client = EthClient::try_from(eth_remote)?;
    let root_cid = eth_client.read().await?;

    // let root_cid = load_root_cid(working_dir.clone())?;
    println!("root_cid: {}", root_cid);
    Ok(())
}

struct HealthReport;

#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    #[error("critical failure")]
    CriticalFailure,
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("eth error: {0}")]
    Eth(#[from] EthClientError),
}
