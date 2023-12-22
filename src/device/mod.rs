use std::path::PathBuf;

use cid::Cid;
use ethers::types::Address;
use ethers::signers::LocalWallet;


// TODO: too many clones here
// TODO: make these mod
pub mod eth;
pub mod ipfs;

use eth::{EthClient, EthClientError, RootCid, RootCidError};
use ipfs::{IpfsClient, IpfsClientError, IpfsGateway, IpfsError, IpfsApi};

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

    /* Root Pointer */

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
    pub async fn update_root_cid(&self,
        previous_root_cid: Cid,
        next_root_cid: Cid
    ) -> Result<Cid, DeviceError> {
        let root_cid = RootCid::new(
            self.eth.clone(),
            self.contract_address,
            Some(self.wallet.clone()),
        )?;
        
        let _root_cid = root_cid.update(
            previous_root_cid,
            next_root_cid
        ).await?;

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
    pub async fn hash(&self, file_path: PathBuf) -> Result<Cid, DeviceError> {
        let file = std::fs::File::open(file_path)?;
        let local = IpfsClient::default();
        let add_response = local.add_with_options(file, ipfs::hash_file_request()).await?;
        let cid = Cid::try_from(add_response.hash)?;
        Ok(cid)
    }

    /// Stage a file against the local ipfs node
    pub async fn stage(&self, file_path: PathBuf) -> Result<Cid, DeviceError> {
        let file = std::fs::File::open(file_path)?;
        let local = IpfsClient::default();
        let add_response = local.add_with_options(file, ipfs::add_file_request()).await?;
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
    pub async fn push(&self, file_path: PathBuf) -> Result<Cid, DeviceError> {
        let file = std::fs::File::open(file_path)?;
        let add_response = self.ipfs.add_with_options(file, ipfs::add_file_request()).await?;
        let cid = Cid::try_from(add_response.hash)?;
        Ok(cid)
    }

    /// Get an ipfs path from the configured ipfs gateway
    pub async fn get(&self, cid: &Cid, path: Option<PathBuf>) -> Result<Vec<u8>, DeviceError> {
        let data = self.ipfs_gateway.get(cid, path).await?;
        Ok(data)
    }
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
}
