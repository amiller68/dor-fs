use std::str::FromStr;

use cid::Cid;
use ethers::types::Address;

pub use crate::eth::{EthClient, EthClientError, EthRemote, RootCid, RootCidError};
use crate::types::Manifest;

use crate::wasm::env::{APP_CHAIN_ID, APP_CONTRACT_ADDRESS, APP_RPC_URL};
use crate::wasm::utils::gateway_url;

/// One stop shop for reading Store data from IPFS and Ethereum
pub struct WasmDevice {
    /// Address for the contract hosting our RootCid
    contract_address: Address,
    /// EthClient for reading and updating a root cid. The contract address should be
    /// callable from this client
    eth: EthClient,
}

/// One stop shop for coordinating interactions with a given remote configuration
impl WasmDevice {
    pub fn new() -> Result<Self, WasmDeviceError> {
        let contract_address =
            Address::from_str(APP_CONTRACT_ADDRESS).map_err(|_e| WasmDeviceError::InvalidContractAddress(APP_CONTRACT_ADDRESS.to_string()))?;
        let chain_id = u32::from_str(APP_CHAIN_ID)?;
        let eth_remote = EthRemote {
            rpc_url: APP_RPC_URL.parse()?,
            chain_id,
        };
        let eth = EthClient::try_from(eth_remote)?;
        Ok(Self {
            contract_address,
            eth,
        })
    }

    /* Dor Store Helpers */

    /// Read a Block by its Cid as a Manifest from Ipfs
    /// # Args
    /// - cid: The cid of the Manifest object
    /// - remote: whether to read against the remote of local IPFS client
    pub async fn read_manifest(&self, cid: &Cid) -> Result<Manifest, WasmDeviceError> {
        let manifest_data = self.read_ipfs_gateway_data(cid).await?;
        let manifest = serde_json::from_slice(&manifest_data)?;
        Ok(manifest)
    }

    /* Eth Helpers */

    /// Get the chain id in use
    pub fn chain_id(&self) -> u32 {
        self.eth.chain_id()
    }

    /// Read the root cid from the eth remote
    pub async fn read_root_cid(&self) -> Result<Cid, WasmDeviceError> {
        let root_cid = RootCid::new(self.eth.clone(), self.contract_address, None)?;
        let root_cid = root_cid.read().await?;
        Ok(root_cid)
    }

    /* Ipfs Helpers */

    /// Read a Cid from the configured Ipfs Gateway
    /// # Args
    /// - cid: the cid to read
    /// - path: Optional path parameter if the Cid points to a unix-fs directory
    pub async fn read_ipfs_gateway_data(
        &self,
        cid: &Cid,
    ) -> Result<Vec<u8>, WasmDeviceError> {
        let url = gateway_url(cid);
        let resp = reqwest::get(url).await?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WasmDeviceError {
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("eth error: {0}")]
    EthClient(#[from] EthClientError),
    #[error("root cid error: {0}")]
    RootCid(#[from] RootCidError),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("int error: {0}")]
    Int(#[from] std::num::ParseIntError),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("url error: {0}")]
    Url(#[from] url::ParseError),
    #[error("invalid contract address: {0}")]
    InvalidContractAddress(String),
}
