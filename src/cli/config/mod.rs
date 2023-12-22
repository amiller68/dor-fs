mod on_disk_default;
mod on_disk_device;

use on_disk_default::OnDiskDefault;
use on_disk_device::OnDiskDevice;

use std::collections::HashMap;
use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cid::Cid;
use ethers::signers::{LocalWallet, Wallet};
use serde::{Deserialize, Serialize};

use crate::ipfs::{IpfsClient, IpfsError, IpfsRemote};
use crate::root_cid::{
    EthClient as RootCidClient, EthClientError as RootCidClientError, EthRemote,
};

use super::args::{Args, ConfigureCreateSubcommand, ConfigureSetSubcommand, ConfigureSubcommand};

// Cli Configuration Constanst

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
pub const DEFAULT_LOCAL_DOT_CHANGELOG: &str = "changes.json";

#[derive(Debug)]
pub struct Config {
    /// Working dir -- defaults to the current working dir
    working_dir: PathBuf,

    /// Path to the device config on disk
    device_alias: Option<String>,

    /// Admin key for managing the root cid contract
    /// Required for initializing a new root cid contract or device
    /// Cannot be on disk defaults or in env
    /// Should be the key that deployed the contract
    admin_key_string: Option<String>,
}

#[allow(dead_code)]
impl Config {
    /// Parse the config from args, env, and on disk defaults appropriate
    /// Takes priority: args > env > on disk defaults
    pub fn parse_args(args: &Args) -> Result<Self, ConfigError> {
        let on_disk_config = OnDiskDefault::load()?;

        let device_alias = on_disk_config.device_alias();

        let working_dir = match args.working_dir {
            Some(wd) => wd,
            None => PathBuf::from("./"),
        };

        let admin_key_string = args.admin_key.clone();

        Ok(Self { admin_key_string })
    }

    // TODO: read device from on disk, read and update state transitions

    pub fn set_device(alias: String) -> Result<(), ConfigError> {
        let on_disk_config = OnDiskDefault::load()?;
        on_disk_config.set_device_alias(alias)
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
}

/// Grab config path
pub fn xdg_config_home() -> Result<PathBuf, ConfigError> {
    // Construct
    let path = PathBuf::from(XDG_PATH.replace(
        '~',
        &env::var("HOME").map_err(|_| ConfigError::MissingEnvVar("HOME".to_string()))?,
    ));
    // Return
    Ok(path)
}
