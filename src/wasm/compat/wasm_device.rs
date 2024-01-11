use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use cid::Cid;
use ethers::types::Address;
use futures_util::stream::TryStreamExt;

pub use crate::device::{EthClient, EthClientError, EthRemote, RootCid, RootCidError};
pub use crate::device::{IpfsError, IpfsGateway};
use crate::types::Manifest;

use crate::wasm::env::{APP_CHAIN_ID, APP_CONTRACT_ADDRESS, APP_IPFS_GATEWAY_URL, APP_RPC_URL};

/// One stop shop for reading Store data from IPFS and Ethereum
pub struct WasmDevice {
    /// Address for the contract hosting our RootCid
    contract_address: Address,
    /// IpfsGateway for pulling data from a public gateway
    ipfs_gateway: IpfsGateway,
    /// EthClient for reading and updating a root cid. The contract address should be
    /// callable from this client
    eth: EthClient,
}

/// One stop shop for coordinating interactions with a given remote configuration
impl WasmDevice {
    pub fn new() -> Result<Self, WasmDeviceError> {
        let contract_address =
            Address::from_str(APP_CONTRACT_ADDRESS).expect("invalid contract address");
        let chain_id = u16::from_str(APP_CHAIN_ID)?;
        let ipfs_gateway_url = APP_IPFS_GATEWAY_URL;
        let ipfs_gateway = IpfsGateway::new(ipfs_gateway_url.parse()?);
        let eth_remote = EthRemote {
            rpc_url: APP_RPC_URL.parse()?,
            chain_id,
        };
        let eth = EthClient::try_from(eth_remote)?;
        Ok(Self {
            contract_address,
            eth,
            ipfs_gateway,
        })
    }

    /* Dor Store Helpers */

    /// Read a Block by its Cid as a Manifest from Ipfs
    /// # Args
    /// - cid: The cid of the Manifest object
    /// - remote: whether to read against the remote of local IPFS client
    pub async fn read_manifest(&self, cid: &Cid) -> Result<Manifest, WasmDeviceError> {
        let manifest_data = self.read_ipfs_gateway_data(cid, None).await?;
        let manifest = serde_json::from_slice(&manifest_data)?;
        Ok(manifest)
    }

    /* Eth Helpers */

    /// Get the chain id in use
    pub fn chain_id(&self) -> u16 {
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
        path: Option<PathBuf>,
    ) -> Result<Vec<u8>, WasmDeviceError> {
        let data = self.ipfs_gateway.get(cid, path).await?;
        Ok(data)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WasmDeviceError {
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("ipfs error: {0}")]
    Ipfs(#[from] IpfsError),
    #[error("eth error: {0}")]
    EthClient(#[from] EthClientError),
    #[error("root cid error: {0}")]
    RootCid(#[from] RootCidError),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("int error: {0}")]
    Int(#[from] std::num::ParseIntError),
    #[error("url error: {0}")]
    Url(#[from] url::ParseError),
}
