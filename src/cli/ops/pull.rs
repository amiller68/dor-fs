use std::io::Write;
use std::path::PathBuf;

use cid::Cid;
use futures_util::TryStreamExt;

use crate::cli::config::{Config, ConfigError};
use crate::ipfs::{IpfsApi, IpfsClient, IpfsClientError, IpfsError};
use crate::root_cid::{EthClient, EthClientError};
use crate::types::DorStore;

use super::diff::{file_cid, DiffError};
use super::utils::{save_root_cid, save_dor_store, load_root_cid, load_dor_store};

// TODO: eth
pub async fn pull(config: &Config, working_dir: PathBuf) -> Result<(), PullError> {
    let eth_client = match config.eth_remote() {
        Some(eth_remote) => EthClient::try_from(eth_remote.clone())?,
        None => {
            return Err(PullError::MissingEthRemote);
        }
    };    
    let remote_ipfs_client = match config.ipfs_remote() {
        Some(ipfs_remote) => IpfsClient::try_from(ipfs_remote.clone())?,
        None => {
            return Err(PullError::MissingIpfsRemote);
        }
    };

    let _root_cid = load_root_cid(working_dir.clone())?;
    let root_cid = eth_client.read().await?;
    if root_cid == Cid::default() || root_cid == _root_cid {
        tracing::info!("root cid is up to date");
    }
    
    let _dor_store = load_dor_store(working_dir.clone())?;
    let dor_store = pull_dor_store(&root_cid, &remote_ipfs_client).await?;
    if dor_store == _dor_store {
        tracing::info!("dor store is up to date");
    }
    
    let objects = dor_store.objects();

    for (path, object) in objects.iter() {
        if !path_needs_pull(path, object.cid()).await? {
            continue;
        }

        // Download the block from the remote, mkdir -p the path, write the block to the path
        let block_data = pull_block(object.cid(),  &remote_ipfs_client).await?;
        let mut object_path = working_dir.join(path);
        object_path.pop();
        std::fs::create_dir_all(object_path)?;
        let mut file = std::fs::File::create(working_dir.join(path))?;
        file.write_all(&block_data)?;
    }

    save_dor_store(working_dir.clone(), &dor_store)?;
    save_root_cid(working_dir.clone(), &root_cid)?;
    Ok(())
}

async fn pull_dor_store(
    root_cid: &Cid,
    remote_ipfs_client: &IpfsClient,
) -> Result<DorStore, PullError> {
    let dor_store_data = pull_block(root_cid, remote_ipfs_client).await?;
    let dor_store = serde_json::from_slice(&dor_store_data)?;
    Ok(dor_store)
}

async fn path_needs_pull(path: &PathBuf, cid: &Cid) -> Result<bool, PullError> {
    if !path.exists() {
        return Ok(true);
    }
    let file_cid = file_cid(path.clone()).await?;
    if file_cid != *cid {
        return Ok(true);
    }
    Ok(false)
}

// TODO: you should really stream this ya know
// TODO: you should try reading locally first
/// Attempt to pull a block from the remote
/// Return the block data as a Vec<u8>
async fn pull_block(
    cid: &Cid,
    remote_ipfs_client: &IpfsClient,
) -> Result<Vec<u8>, PullError> {
    let block_stream = remote_ipfs_client.block_get(&cid.to_string());
    let block_data = block_stream
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await?;
    Ok(block_data)
}

#[derive(Debug, thiserror::Error)]
pub enum PullError {
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("diff error: {0}")]
    Diff(#[from] DiffError),
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
    IpfsClient(#[from] IpfsClientError),
    #[error("missing ipfs remote")]
    MissingIpfsRemote,
    #[error("missing eth remote")]
    MissingEthRemote,
}
