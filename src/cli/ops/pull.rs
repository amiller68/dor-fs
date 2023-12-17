use std::path::PathBuf;
use std::str::FromStr;

use cid::Cid;

use crate::cli::config::{Config, ConfigError, DEFAULT_MFS_ROOT};
use crate::ipfs::{IpfsApi, IpfsClient, IpfsError};
use crate::root_cid::{EthClient, EthClientError};

// TODO: eth
pub async fn pull(config: &Config) -> Result<(), PushError> {
    // let change_log = load_change_log(working_dir.clone())?;
    let local_ipfs_client = IpfsClient::default();

    let mfs_root = PathBuf::from(format!("/{}", DEFAULT_MFS_ROOT));

    let root_stat = local_ipfs_client
        .files_stat(&mfs_root.to_str().unwrap())
        .await?;
    let root_hash = root_stat.hash;

    let remote_ipfs_client = match config.ipfs_remote() {
        Some(ipfs_remote) => IpfsClient::try_from(ipfs_remote.clone())?,
        None => {
            return Err(PushError::MissingIpfsRemote);
        }
    };

    let remote_root_cid_client = match config.eth_remote() {
        Some(eth_remote) => EthClient::try_from(eth_remote.clone())?,
        None => {
            return Err(PushError::MissingEthRemote);
        }
    };

    // Recursive pin the root
    let remote_root_stat = remote_ipfs_client.pin_add(&root_hash, true).await?;
    let remote_root_hash = remote_root_stat.pins[0].clone();
    let remote_root_cid = Cid::from_str(&remote_root_hash)?;
    tracing::info!("Pushing root: {}", remote_root_cid);

    // Update the root cid
    // let previous_root_cid = config.root_cid();

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum PushError {
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("eth client error: {0}")]
    EthClient(#[from] EthClientError),
    #[error("fs-tree error: {0}")]
    FsTree(#[from] fs_tree::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse diff: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("could not strip prefix: {0}")]
    PathPrefix(#[from] std::path::StripPrefixError),
    #[error("ipfs error: {0}")]
    Ipfs(#[from] IpfsError),
    #[error("ipfs backend error: {0}")]
    IpfsBackend(#[from] ipfs_api_backend_hyper::Error),
    #[error("missing ipfs remote")]
    MissingIpfsRemote,
    #[error("missing eth remote")]
    MissingEthRemote,
}
