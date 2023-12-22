use std::fmt::Display;

use cid::Cid;

use crate::cli::config::{Config, ConfigError};

// TODO: check if all services are reachable, print out relevant config info in a pretty way
pub async fn health(config: &Config) -> Result<(), HealthError> {
    let device = config.device()?;
    
    let alias = config.device_alias();
    let root_cid = match device.get_root_cid().await {
        Ok(root_cid) => Some(root_cid),
        Err(_) => None,
    };
    let eth_online = root_cid.is_some();

    let local_ipfs_online = match device.local_id().await {
        Ok(_) => true,
        Err(_) => false,
    };

    let ipfs_online = match device.remote_id().await {
        Ok(_) => true,
        Err(_) => false,
    };

    let report = HealthReport {
        alias,
        root_cid,
        local_ipfs_online,
        ipfs_online,
        eth_online,
    };

    println!("{}", report);
    Ok(())
}

struct HealthReport {
    alias: Option<String>,
    root_cid: Option<Cid>,
    local_ipfs_online: bool,
    ipfs_online: bool,
    eth_online: bool,
}

impl Display for HealthReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alias = match &self.alias {
            Some(alias) => alias,
            None =>  return write!(f, "no device configured"),
        };

        let root_cid = match &self.root_cid {
            Some(root_cid) => root_cid.to_string(),
            None => "not configured".to_string(),
        };
        let local_ipfs_online = if self.local_ipfs_online {
            "online"
        } else {
            "offline"
        };
        let ipfs_online = if self.ipfs_online {
            "online"
        } else {
            "offline"
        };
        let eth_online = if self.eth_online {
            "online"
        } else {
            "offline"
        };

        write!(
            f,
            "alias: {}, root_cid: {}, local_ipfs: {}, ipfs: {}, eth: {}",
            alias, root_cid, local_ipfs_online, ipfs_online, eth_online
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    #[error("critical failure")]
    CriticalFailure,
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
