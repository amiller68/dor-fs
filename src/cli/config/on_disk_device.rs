use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cid::Cid;
use ethers::types::{Address, LocalWallet};
use serde::{Deserialize, Serialize};

use crate::device::eth::EthRemote;
use crate::device::ipfs::IpfsRemote;
use crate::types::DorStore;

use super::{
    xdg_home_path, ConfigError, BASE_DOR_STORE_NAME, DEVICE_CONFIG_NAME, DEVICE_KEYSTORE_NAME,
    ROOT_CID_NAME,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// An OnDiskDevice Configuration
/// Specifies both connection to remote, and api for managing device state via an alias
pub struct OnDiskDevice {
    // Remote configuration
    /// Address for the contract publishing our root cid
    contract_address: Address,
    /// Connection to an EthRemote
    eth_remote: EthRemote,
    /// Connection to an IpfsRemote
    ipfs_remote: IpfsRemote,
}

impl OnDiskDevice {
    /// Create a new on disk device
    pub fn new(
        alias: String,
        ipfs_remote: IpfsRemote,
        eth_remote: EthRemote,
        contract_address: Address,
    ) -> Result<Self, ConfigError> {
        todo!()
    }

    /// Return the on disk device
    pub fn load(alias: String) -> Result<Self, ConfigError> {
        todo!()
    }

    /// Update the on disk device configuration
    pub fn update(
        alias: String,
        ipfs_remote: Option<IpfsRemote>,
        eth_remote: Option<EthRemote>,
        contract_address: Option<Address>,
    ) -> Result<Self, ConfigError> {
        todo!()
    }

    /// Read the keystore from disk for the device
    pub fn keystore(alias: String) -> Result<LocalWallet, ConfigError> {
        todo!()
    }

    /// Read the root cid from disk for the device
    pub fn root_cid(alias: String) -> Result<Cid, ConfigError> {
        todo!()
    }

    /// Set the root cid on disk for the device
    pub fn set_root_cid(alias: String, cid: Cid) -> Result<(), ConfigError> {
        todo!()
    }

    /// Read the base dor store metadata from disk for the device
    pub fn base(alias: String) -> Result<DorStore, ConfigError> {
        todo!()
    }

    /// Set the base dor store metadata for the device
    pub fn set_base(alias: String, base: DorStore) -> Result<(), ConfigError> {
        todo!()
    }

    /// Save the config to its default location on disk
    fn save(&self) -> Result<(), ConfigError> {
        let xdg_path = xdg_config_home()?;
        let config_path = xdg_path.join(DEFAULT_CONFIG_NAME);

        let config_json = serde_json::to_string(&self)?;
        let mut config_file = File::create(config_path)?;
        config_file.write_all(config_json.as_bytes())
    }
}
