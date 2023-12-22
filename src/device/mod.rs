use ethers::signers::LocalWallet;

// TODO: make these mod
pub mod eth;
pub mod ipfs;

use eth::{EthClient, EthClientError};
use ipfs::{IpfsClient, IpfsClientError, IpfsGateway};

/// Device:
/// Responsible for configuring a connection against
/// a remote (ipfs + eth)
pub struct Device {
    pub ipfs: IpfsClient,
    pub ipfs_gateway: IpfsGateway,
    pub eth: EthClient,
    pub wallet: LocalWallet,
}

impl Device {
    pub fn new(
        ipfs: IpfsClient,
        ipfs_gateway: IpfsGateway,
        eth: EthClient,
        wallet: LocalWallet,
    ) -> Self {
        Self {
            ipfs,
            ipfs_gateway,
            eth,
            wallet,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("ipfs error: {0}")]
    IpfsClient(#[from] IpfsClientError),
    #[error("eth error: {0}")]
    EthClient(#[from] EthClientError),
}
