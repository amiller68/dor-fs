use std::fs::create_dir_all;
use std::{env, io::Write, path::PathBuf};

use cid::Cid;
use ethers::signers::LocalWallet;
use ethers::types::Address;
use fs_tree::FsTree;

use crate::cli::changes::ChangeLog;
use crate::device::{Device, EthRemote, IpfsRemote};
use crate::types::DorStore;

mod on_disk_default;
pub mod on_disk_device;

use on_disk_default::OnDiskDefault;
use on_disk_device::OnDiskDevice;

use super::args::Args;

// Cli Configuration Constants

// path to application data on disk
const XDG_PATH: &str = "~/.config/dor-store";
// name to lookup on disk defaults under
pub const DEFAULT_CONFIG_NAME: &str = "default.json";
// name to lookup on disk keystore under a given device alias
pub const DEVICE_KEYSTORE_NAME: &str = "keystore.json";
// name to lookup on disk device under a given device alias
pub const DEVICE_CONFIG_NAME: &str = "device.json";
// name to lookup on disk root cid under a given device alias
pub const ROOT_CID_NAME: &str = "root";
// name to lookup on disk base dor-store under a given device alias
pub const BASE_DOR_STORE_NAME: &str = "base.json";

// path to folder containing local changes tracking in the given working dir
pub const DEFAULT_LOCAL_DOT_DIR: &str = ".fs";
// name to lookup change log within a dot dir
pub const CHANGE_LOG_NAME: &str = "changes.json";

#[derive(Debug)]
pub struct Config {
    /// Working dir -- defaults to the current working dir
    working_dir: PathBuf,

    /// Alias for the device to use
    device_alias: Option<String>,

    /// Admin key for managing the root cid contract
    /// Required for initializing a new root cid contract or device
    /// Cannot be on disk defaults or in env
    /// Should be the key that deployed the contract
    admin_key_string: Option<String>,
}

// TODO: should isolate side effects from config
#[allow(dead_code)]
impl Config {
    /// Parse the config from args and on disk defaults
    pub fn parse_args(args: &Args) -> Result<Self, ConfigError> {
        let on_disk_config = OnDiskDefault::load()?;

        let device_alias = on_disk_config.device_alias();

        let maybe_dir = args.dir.clone();
        let working_dir_str = match maybe_dir {
            Some(s) => s,
            None => ".".to_string(),
        };
        let working_dir = PathBuf::from(working_dir_str);

        let admin_key_string = args.admin_key.clone();

        Ok(Self {
            working_dir,
            device_alias,
            admin_key_string,
        })
    }

    pub fn working_dir(&self) -> &PathBuf {
        &self.working_dir
    }

    /// Get the next fs-tree from the working directory           
    pub fn fs_tree(&self) -> Result<FsTree, ConfigError> {
        let working_dir = self.working_dir();
        let dot_dir = PathBuf::from(DEFAULT_LOCAL_DOT_DIR);
        // Read Fs-tree at dir or pwd, stripping off the local dot directory
        let next = match fs_tree::FsTree::read_at(working_dir.to_str().unwrap())? {
            fs_tree::FsTree::Directory(mut d) => {
                let _res = &d.remove_entry(&dot_dir);
                fs_tree::FsTree::Directory(d)
            }
            _ => {
                return Err(ConfigError::DotDirNotADirectory);
            }
        };
        Ok(next)
    }
    pub fn with_device_alias(&mut self, alias: String) -> &Self {
        self.device_alias = Some(alias);
        self
    }

    /* Methods */

    pub fn list_on_disk_devices() -> Result<Vec<OnDiskDevice>, ConfigError> {
        OnDiskDevice::list()
    }

    pub fn create_on_disk_device(
        alias: String,
        contract_address: Address,
        ipfs_remote: IpfsRemote,
        eth_remote: EthRemote,
    ) -> Result<OnDiskDevice, ConfigError> {
        let device = OnDiskDevice::new(alias, ipfs_remote, eth_remote, contract_address)?;
        Ok(device)
    }

    /* Members */

    pub fn init(&self) -> Result<(), ConfigError> {
        let alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;
        let dot_path = self.working_dir.join(DEFAULT_LOCAL_DOT_DIR);

        // Check if the dot path exists
        if dot_path.exists() {
            return Ok(());
        }
        create_dir_all(dot_path.clone())?;

        let root_cid = self.root_cid()?;
        let base = self.base()?;

        let change_log_path = dot_path.join(CHANGE_LOG_NAME);
        let change_log = ChangeLog::new(alias, &base, &root_cid);
        let change_log_str = serde_json::to_string_pretty(&change_log)?;
        let mut change_log_file = std::fs::File::create(change_log_path)?;
        change_log_file.write_all(change_log_str.as_bytes())?;
        Ok(())
    }

    pub fn change_log(&self) -> Result<ChangeLog, ConfigError> {
        let dot_path = self.working_dir.join(DEFAULT_LOCAL_DOT_DIR);
        let change_log_path = dot_path.join(CHANGE_LOG_NAME);

        let alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;

        if !change_log_path.exists() {
            return Err(ConfigError::ChangeLogNotFound);
        }

        let change_log_str = std::fs::read_to_string(change_log_path)?;
        let change_log: ChangeLog = serde_json::from_str(&change_log_str)?;

        if change_log.manager_alias() != &alias {
            return Err(ConfigError::ChangeLogNotFound);
        }

        Ok(change_log)
    }

    pub fn set_change_log(&self, change_log: ChangeLog) -> Result<(), ConfigError> {
        let dot_path = self.working_dir.join(DEFAULT_LOCAL_DOT_DIR);
        let change_log_path = dot_path.join(CHANGE_LOG_NAME);

        let change_log_str = serde_json::to_string_pretty(&change_log)?;
        let mut change_log_file = std::fs::File::create(change_log_path)?;
        change_log_file.write_all(change_log_str.as_bytes())?;

        Ok(())
    }

    pub fn root_cid(&self) -> Result<Cid, ConfigError> {
        let device_alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;
        let cid = OnDiskDevice::root_cid(device_alias)?;
        Ok(cid)
    }

    pub fn set_root_cid(&self, cid: &Cid) -> Result<(), ConfigError> {
        let device_alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;
        OnDiskDevice::set_root_cid(device_alias, cid)
    }

    pub fn base(&self) -> Result<DorStore, ConfigError> {
        let device_alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;
        let base = OnDiskDevice::base(device_alias)?;
        Ok(base)
    }

    pub fn set_base(&self, base: &DorStore) -> Result<(), ConfigError> {
        let device_alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;
        OnDiskDevice::set_base(device_alias, base)
    }

    pub fn device(&self) -> Result<Device, ConfigError> {
        let device_alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;
        let device_config = OnDiskDevice::load(device_alias)?;
        let device = Device::try_from(device_config).unwrap();
        match self.admin_key_string.clone() {
            Some(admin_key_string) => {
                let admin_key = admin_key_string
                    .parse::<LocalWallet>()
                    .expect("invalid key");
                let device = device.with_wallet(admin_key);
                Ok(device)
            }
            None => Ok(device),
        }
    }

    pub fn device_alias(&self) -> Option<String> {
        self.device_alias.clone()
    }

    pub fn set_device(alias: String) -> Result<(), ConfigError> {
        let mut on_disk_config = OnDiskDefault::load()?;
        on_disk_config.set_device_alias(alias)
    }

    pub fn on_disk_device(&self) -> Result<OnDiskDevice, ConfigError> {
        let device_alias = self.device_alias.clone().ok_or(ConfigError::NoSetDevice)?;
        let device_config = OnDiskDevice::load(device_alias)?;
        Ok(device_config)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to create directory: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("no set device")]
    NoSetDevice,
    #[error("dot dir is not a directory")]
    DotDirNotADirectory,
    #[error("device alias not found: {0}")]
    DeviceNotFound(String),
    // #[error("device exists: {0}")]
    // DeviceExists(String),
    #[error("change log not found")]
    ChangeLogNotFound,
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("fs tree error: {0}")]
    FsTree(#[from] fs_tree::Error),
}

/// Grab config path
pub fn xdg_config_home() -> Result<PathBuf, ConfigError> {
    // Construct
    let path =
        PathBuf::from(XDG_PATH.replace('~', &env::var("HOME").unwrap_or_else(|_| "/".to_string())));
    // Return
    Ok(path)
}
