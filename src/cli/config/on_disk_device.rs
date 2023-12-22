use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cid::Cid;
use ethers::signers::{LocalWallet, Wallet};
use ethers::types::Address;
use serde::{Deserialize, Serialize};

use crate::device::eth::{EthClient, EthClientError, EthRemote};
use crate::device::ipfs::{IpfsClient, IpfsError, IpfsGateway, IpfsRemote};
use crate::device::Device;
use crate::types::DorStore;

use super::{
    xdg_config_home, ConfigError, BASE_DOR_STORE_NAME, DEVICE_CONFIG_NAME, DEVICE_KEYSTORE_NAME,
    ROOT_CID_NAME,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// An OnDiskDevice Configuration
/// Specifies both connection to remote, and api for managing device state via an alias
pub struct OnDiskDevice {
    alias: String,

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
        let mut rng = rand::thread_rng();
        let xdg_path = xdg_config_home()?;
        let device_path = xdg_path.join(alias.clone());

        // TODO: re-enable this check
        // Check if the device already exists
        // if device_path.exists() {
        //     return Err(ConfigError::DeviceExists(alias.clone()));
        // }

        create_dir_all(&device_path)?;

        let _wallet =
            Wallet::new_keystore(&device_path, &mut rng, "", Some(DEVICE_KEYSTORE_NAME)).unwrap();

        let config = Self {
            alias: alias.clone(),
            contract_address,
            eth_remote,
            ipfs_remote,
        };

        let cid = Cid::default();
        let base = DorStore::default();

        Self::set_root_cid(alias.clone(), &cid)?;
        Self::set_base(alias.clone(), &base)?;

        config.save(alias.clone())?;

        Ok(config)
    }

    /// Return the on disk device
    pub fn load(alias: String) -> Result<Self, ConfigError> {
        let device_path = device_path(alias.clone())?;
        let config_path = device_path.join(DEVICE_CONFIG_NAME);
        if !config_path.exists() {
            return Err(ConfigError::DeviceNotFound(alias.clone()));
        }
        let config = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config)?;
        Ok(config)
    }

    /// alias
    pub fn alias(&self) -> String {
        self.alias.clone()
    }

    /// Return a list of all on disk devices
    pub fn list() -> Result<Vec<Self>, ConfigError> {
        let xdg_path = xdg_config_home()?;
        let mut devices = Vec::new();
        for entry in std::fs::read_dir(xdg_path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let alias = path.file_name().unwrap().to_str().unwrap().to_string();
            let device = Self::load(alias.clone())?;
            devices.push(device);
        }
        Ok(devices)
    }

    /// Update the on disk device configuration
    pub fn _update(
        alias: String,
        ipfs_remote: Option<IpfsRemote>,
        eth_remote: Option<EthRemote>,
        contract_address: Option<Address>,
    ) -> Result<Self, ConfigError> {
        let mut device = Self::load(alias.clone())?;

        if let Some(ipfs_remote) = ipfs_remote {
            device.ipfs_remote = ipfs_remote;
        }
        if let Some(eth_remote) = eth_remote {
            device.eth_remote = eth_remote;
        }
        if let Some(contract_address) = contract_address {
            device.contract_address = contract_address;
        }

        device.save(alias.clone())?;
        Ok(device)
    }

    /// Read the keystore from disk for the device
    pub fn keystore(alias: String) -> Result<LocalWallet, ConfigError> {
        let device_path = device_path(alias.clone())?;
        let keystore_path = device_path.join(DEVICE_KEYSTORE_NAME);
        let wallet = LocalWallet::decrypt_keystore(keystore_path, "").unwrap();
        Ok(wallet)
    }

    /// Read the root cid from disk for the device
    pub fn root_cid(alias: String) -> Result<Cid, ConfigError> {
        let device_path = device_path(alias.clone())?;
        let root_cid_path = device_path.join(ROOT_CID_NAME);
        let root_cid_str = std::fs::read_to_string(root_cid_path)?;
        let root_cid = Cid::from_str(&root_cid_str)?;
        Ok(root_cid)
    }

    /// Set the root cid on disk for the device
    pub fn set_root_cid(alias: String, cid: &Cid) -> Result<(), ConfigError> {
        let device_path = device_path(alias.clone())?;
        let root_cid_path = device_path.join(ROOT_CID_NAME);
        let root_cid_str = cid.to_string();
        let mut root_cid_file = File::create(root_cid_path)?;
        root_cid_file.write_all(root_cid_str.as_bytes())?;
        Ok(())
    }

    /// Read the base dor store metadata from disk for the device
    pub fn base(alias: String) -> Result<DorStore, ConfigError> {
        let device_path = device_path(alias.clone())?;
        let base_path = device_path.join(BASE_DOR_STORE_NAME);
        let base_str = std::fs::read_to_string(base_path)?;
        let base = serde_json::from_str(&base_str)?;
        Ok(base)
    }

    /// Set the base dor store metadata for the device
    pub fn set_base(alias: String, base: &DorStore) -> Result<(), ConfigError> {
        let device_path = device_path(alias.clone())?;
        let base_path = device_path.join(BASE_DOR_STORE_NAME);
        let base_str = serde_json::to_string(&base)?;
        let mut base_file = File::create(base_path)?;
        base_file.write_all(base_str.as_bytes())?;
        Ok(())
    }

    /// Save the config to its default location on disk
    fn save(&self, alias: String) -> Result<(), ConfigError> {
        let device_path = device_path(alias.clone())?;
        let config_path = device_path.join(DEVICE_CONFIG_NAME);

        let config_json = serde_json::to_string(&self)?;
        let mut config_file = File::create(config_path)?;
        config_file.write_all(config_json.as_bytes())?;
        Ok(())
    }
}

impl TryFrom<OnDiskDevice> for Device {
    type Error = OnDiskDeviceError;

    fn try_from(on_disk_device: OnDiskDevice) -> Result<Self, Self::Error> {
        let eth_remote = on_disk_device.eth_remote;
        let ipfs_remote = on_disk_device.ipfs_remote;
        let contract_address = on_disk_device.contract_address;
        let alias = on_disk_device.alias;

        let eth = EthClient::try_from(eth_remote)?;
        let ipfs = IpfsClient::try_from(ipfs_remote.clone())?;
        let ipfs_gateway = IpfsGateway::from(ipfs_remote);
        let wallet = OnDiskDevice::keystore(alias.clone())?;

        let device = Device {
            contract_address,
            eth,
            ipfs,
            ipfs_gateway,
            wallet,
        };

        Ok(device)
    }
}

impl Display for OnDiskDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let eth_remote = self.eth_remote.clone();
        let ipfs_remote = self.ipfs_remote.clone();
        let contract_address = self.contract_address;
        let alias = self.alias.clone();

        write!(
            f,
            "alias: {}, eth_remote: {}, ipfs_remote: {}, contract_address: {}",
            alias, eth_remote, ipfs_remote, contract_address
        )
    }
}

fn device_path(alias: String) -> Result<PathBuf, ConfigError> {
    let xdg_path = xdg_config_home()?;
    let device_path = xdg_path.join(alias);
    Ok(device_path)
}

#[derive(Debug, thiserror::Error)]
pub enum OnDiskDeviceError {
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("eth client error: {0}")]
    EthClient(#[from] EthClientError),
    #[error("ipfs error: {0}")]
    Ipfs(#[from] IpfsError),
    #[error("wallet error: {0}")]
    Wallet(#[from] ethers::signers::WalletError),
}
