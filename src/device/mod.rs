use ethers::types::LocalWallet;

// TODO: make these mod
pub mod eth;
pub mod ipfs;

use eth::{EthClient, EthClientError};
use ipfs::{IpfsClient, IpfsClientError};

use crate::cli::{Config, ConfigError};
use crate::types::DorStore;

/// Device:
/// Responsible for configuring a connection against
/// a remote (ipfs + eth)
pub struct Device {
    ipfs: IpfsClient,
    ipfs_gateway: IpfsGateway,
    eth: EthClient,
    wallet: LocalWallet,
}

#[derive(Debug, this_error::Error)]
pub enum DeviceError {
    #[error("ipfs error: {0}")]
    IpfsClient(#[from] IpfsClientError),
    #[error("eth error: {0}")]
    EthClient(#[from] EthClientError),
}
