use std::io::Write;
use std::path::PathBuf;

use cid::Cid;
use ethers::signers::LocalWallet;
use ethers::types::Address;
use futures_util::stream::TryStreamExt;

// TODO: too many clones here
// TODO: make these mod
pub mod eth;
pub mod ipfs;

use eth::{EthClient, EthClientError, RootCid};
use ipfs::{IpfsApi, IpfsClient, IpfsClientError, IpfsError, IpfsGateway};

use crate::types::DorStore;

/// Device:
/// Responsible for configuring a connection against
/// a remote (ipfs + eth)
pub struct Device {
    pub contract_address: Address,
    pub ipfs: IpfsClient,
    pub ipfs_gateway: IpfsGateway,
    pub eth: EthClient,
    pub wallet: LocalWallet,
}

impl Device {
    /// Set the LocalWallet
    pub fn with_wallet(mut self, wallet: LocalWallet) -> Self {
        self.wallet = wallet;
        self
    }

    /* Store Helpers */

    pub async fn pull_dor_store(&self, root_cid: &Cid) -> Result<DorStore, DeviceError> {
        let dor_store_data = self.pull_block(root_cid).await?;
        let dor_store = serde_json::from_slice(&dor_store_data)?;
        Ok(dor_store)
    }

    pub async fn push_dor_store(&self, dor_store: &DorStore) -> Result<Cid, DeviceError> {
        let dor_store_data = serde_json::to_vec(&dor_store)?;
        let dor_store_data = std::io::Cursor::new(dor_store_data);
        let cid = self
            .ipfs
            .add_with_options(dor_store_data, ipfs::add_file_request())
            .await?;
        let cid = Cid::try_from(cid.hash)?;
        Ok(cid)
    }

    pub async fn hash_dor_store(&self, dor_store: &DorStore) -> Result<Cid, DeviceError> {
        let dor_store_data = serde_json::to_vec(&dor_store)?;
        let dor_store_data = std::io::Cursor::new(dor_store_data);
        let cid = self
            .ipfs
            .add_with_options(dor_store_data, ipfs::add_file_request())
            .await?;
        let cid = Cid::try_from(cid.hash)?;
        Ok(cid)
    }

    pub async fn file_needs_pull(&self, path: &PathBuf, cid: &Cid) -> Result<bool, DeviceError> {
        if !path.exists() {
            return Ok(true);
        }
        let hash = self.hash(path).await?;
        if hash == *cid {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /* Root Pointer */

    /// Get the chain id from the remote eth node
    pub fn chain_id(&self) -> u16 {
        self.eth.chain_id()
    }

    /// Get the root cid from the remote eth node
    pub async fn get_root_cid(&self) -> Result<Cid, DeviceError> {
        let root_cid = RootCid::new(
            self.eth.clone(),
            self.contract_address,
            Some(self.wallet.clone()),
        )?;
        let root_cid = root_cid.read().await?;
        Ok(root_cid)
    }

    /// Update the root cid on the remote eth node
    pub async fn update_root_cid(
        &self,
        previous_root_cid: Cid,
        next_root_cid: Cid,
    ) -> Result<Cid, DeviceError> {
        let root_cid = RootCid::new(
            self.eth.clone(),
            self.contract_address,
            Some(self.wallet.clone()),
        )?;

        let _root_cid = root_cid.update(previous_root_cid, next_root_cid).await?;

        // TODO: maybe should wait for emitted event

        Ok(next_root_cid)
    }

    /* Ipfs */

    /// Get the local node Id
    pub async fn local_id(&self) -> Result<String, DeviceError> {
        let local = IpfsClient::default();
        let id_response = local.id(None).await?;
        Ok(id_response.id)
    }

    /// Get the remote node Id
    pub async fn remote_id(&self) -> Result<String, DeviceError> {
        let id_response = self.ipfs.id(None).await?;
        Ok(id_response.id)
    }

    /// Hash a file against the local ipfs node
    pub async fn hash(&self, file_path: &PathBuf) -> Result<Cid, DeviceError> {
        let file = std::fs::File::open(file_path)?;
        let local = IpfsClient::default();
        let add_response = local
            .add_with_options(file, ipfs::hash_file_request())
            .await?;
        let cid = Cid::try_from(add_response.hash)?;
        Ok(cid)
    }

    /// Stage a file against the local ipfs node
    pub async fn stage(&self, file_path: &PathBuf) -> Result<Cid, DeviceError> {
        let file = std::fs::File::open(file_path)?;
        let local = IpfsClient::default();
        let add_response = local
            .add_with_options(file, ipfs::add_file_request())
            .await?;
        let cid = Cid::try_from(add_response.hash)?;
        Ok(cid)
    }

    /// State a cid against the remote ipfs node
    pub async fn remote_stat(&self, cid: &Cid) -> Result<Option<u64>, DeviceError> {
        let cid = cid.to_string();
        let stat_response = self.ipfs.block_stat(&cid);
        match stat_response.await {
            Ok(stat) => Ok(Some(stat.size)),
            Err(IpfsClientError::Api(api_error)) => {
                if api_error.code == 0 && api_error.message == "blockservice: key not found" {
                    Ok(None)
                } else {
                    Err(DeviceError::IpfsClient(IpfsClientError::Api(api_error)))
                }
            }
            Err(e) => Err(DeviceError::IpfsClient(e)),
        }
    }

    /// Push a file to the remote ipfs node
    pub async fn push(&self, file_path: &PathBuf) -> Result<Cid, DeviceError> {
        let file = std::fs::File::open(file_path)?;
        let add_response = self
            .ipfs
            .add_with_options(file, ipfs::add_file_request())
            .await?;
        let cid = Cid::try_from(add_response.hash)?;
        Ok(cid)
    }

    /// Pull a block from the remote ipfs node
    pub async fn pull(&self, cid: &Cid, path: &PathBuf) -> Result<(), DeviceError> {
        let data = self.pull_block(cid).await?;
        let mut object_path = path.clone();
        object_path.pop();
        std::fs::create_dir_all(object_path)?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(&data)?;
        Ok(())
    }

    /// Pull a file from the remote ipfs gateway
    // pub async fn download_file(&self, cid: &Cid, path: &PathBuf) -> Result<(), DeviceError> {
    //     let data = self.get(cid, None).await?;
    //     let mut object_path = path.clone();
    //     object_path.pop();
    //     std::fs::create_dir_all(object_path)?;
    //     let mut file = std::fs::File::create(path)?;
    //     file.write_all(&data)?;
    //     Ok(())
    // }

    /// Return the block data from the remote as a Vec<u8>
    pub async fn pull_block(self: &Self, cid: &Cid) -> Result<Vec<u8>, DeviceError> {
        let block_stream = self.ipfs.block_get(&cid.to_string());
        let block_data = block_stream
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await?;
        Ok(block_data)
    }

    // TODO: this isn't working quite right
    // / Get an ipfs path from the configured ipfs gateway
    // pub async fn get(&self, cid: &Cid, path: Option<PathBuf>) -> Result<Vec<u8>, DeviceError> {
    //     let data = self.ipfs_gateway.get(cid, path).await?;
    //     Ok(data)
    // }
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("ipfs error: {0}")]
    Ipfs(#[from] IpfsError),
    #[error("ipfs error: {0}")]
    IpfsClient(#[from] IpfsClientError),
    #[error("eth error: {0}")]
    EthClient(#[from] EthClientError),
    #[error("root cid error: {0}")]
    RootCid(#[from] eth::RootCidError),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
