use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use ethers::signers::Wallet;
use serde::{Deserialize, Serialize};

use crate::ipfs::IpfsRemote;

use super::args::Args;

const XDG_PATH: &str = "~/.config/krondor-fs";
const DEFAULT_CONFIG_NAME: &str = "defaults.json";
const DEFAULT_DEVICE_KEYSTORE_NAME: &str = "device-keystore.json";
const DEFAULT_LOCAL_DOT_DIR: &str = ".fs";
pub const DEFAULT_LOCAL_DOT_CHANGELOG: &str = "changelog.json";

pub fn working_dot_dir(working_dir: PathBuf) -> Result<PathBuf, ConfigError> {
    // Check if the local config exists in the working directory
    let local_dot_path = local_dot_dir_with_base(&working_dir);
    // If not then this dir has not been cloned
    if !local_dot_path.exists() || !local_dot_path.is_dir() {
        return Err(ConfigError::MissingDotPath(local_dot_path).into());
    }
    Ok(local_dot_path)
}

/// Path to the local dot directory tracking changes to the local filesystem
pub fn dot_dir() -> PathBuf {
    PathBuf::from(DEFAULT_LOCAL_DOT_DIR)
}

/// Path to the local dot directory tracking changes to the local filesystem
/// relative to the working directory
pub fn local_dot_dir_with_base(working_dir_path: &Path) -> PathBuf {
    working_dir_path.join(DEFAULT_LOCAL_DOT_DIR)
}

#[allow(dead_code)]
// TODO: ipfs configuration
#[derive(Debug)]
pub struct Config {
    /// Address of the root cid contract
    /// Should have on disk defaults
    contract_address_string: Option<String>,
    /// Path to the device keystore
    /// Should have on disk defaults
    device_keystore_path: Option<PathBuf>,

    /// Local configuration for preparing and pinning content
    /// Should be a local ipfs node with access to add, mfs,and blocks api
    /// Default is http
    local_ipfs_scheme: String,
    /// Default is localhost
    local_ipfs_host: String,
    /// Default is 5001
    local_ipfs_api_port: String,
    /// Default is 8080
    local_ipfs_gateway_port: String,

    /// Remote configuration for preparing and pinning content
    /// Should be a remote ipfs node with access add and blocks api
    /// Can have on disk defaults
    /// Default is none
    ipfs_remote: Option<IpfsRemote>,

    /// Admin key for managing the root cid contract
    /// Required for initializing a new root cid contract or device
    /// Cannot be on disk defaults or in env
    /// TODO: Metamask workflow
    admin_key_string: Option<String>,
}

#[allow(dead_code)]
impl Config {
    /// Parse the config from args, env, and on disk defaults appropriate
    /// Takes priority: args > env > on disk defaults
    pub fn parse(args: &Args) -> Result<Self, ConfigError> {
        // This is weired but makes the interface clean
        // There's a weird responsibility boundary crossing here but
        if args.command.clone() == super::args::Command::Init {
            tracing::info!("Initializing new on disk config");
            OnDiskConfig::init()?;
            std::process::exit(0);
        }

        // (maybe) Load the on disk config
        let maybe_on_disk_config = match OnDiskConfig::load() {
            Ok(on_disk_config) => Some(on_disk_config),
            Err(ConfigError::MissingConfigFile(_)) => {
                tracing::warn!("No on disk config found");
                None
            }
            Err(e) => {
                tracing::error!("Failed to load on disk config: {:?}", e);
                return Err(e);
            }
        };

        // kinda verbose and ugly
        let contract_address_string = match args.contract.clone() {
            Some(contract_address_string) => Some(contract_address_string),
            None => match env::var("CONTRACT_ADDRESS") {
                Ok(contract_address_string) => Some(contract_address_string),
                Err(_) => match maybe_on_disk_config.clone() {
                    Some(on_disk_config) => on_disk_config.root_cid_contract_address_string(),
                    None => {
                        tracing::warn!("Missing contract address");
                        None
                    }
                },
            },
        };

        let device_keystore_path = match args.device_keystore.clone() {
            Some(device_keystore_path) => Some(device_keystore_path),
            None => match env::var("DEVICE_KEYSTORE") {
                Ok(device_keystore_path) => Some(PathBuf::from(device_keystore_path)),
                Err(_) => match maybe_on_disk_config.clone() {
                    Some(on_disk_config) => Some(on_disk_config.device_keystore_path()),
                    None => {
                        tracing::warn!("Missing device keystore path");
                        None
                    }
                },
            },
        };

        let local_ipfs_scheme = args.local_ipfs_scheme.clone().unwrap_or_else(|| {
            env::var("LOCAL_IPFS_SCHEME").unwrap_or_else(|_| "http".to_string())
        });

        let local_ipfs_host = args.local_ipfs_host.clone().unwrap_or_else(|| {
            env::var("LOCAL_IPFS_HOST").unwrap_or_else(|_| "localhost".to_string())
        });

        let local_ipfs_api_port = args
            .local_ipfs_port
            .clone()
            .unwrap_or_else(|| env::var("LOCAL_IPFS_PORT").unwrap_or_else(|_| "5001".to_string()));

        let local_ipfs_gateway_port = args.local_ipfs_port.clone().unwrap_or_else(|| {
            env::var("LOCAL_IPFS_GATEWAY_PORT").unwrap_or_else(|_| "8080".to_string())
        });

        let ipfs_remote = match maybe_on_disk_config {
            Some(on_disk_config) => on_disk_config.ipfs_remote(),
            None => None,
        };

        let admin_key_string = args.admin_key.clone();

        Ok(Self {
            contract_address_string,
            device_keystore_path,
            local_ipfs_scheme,
            local_ipfs_host,
            local_ipfs_api_port,
            local_ipfs_gateway_port,
            ipfs_remote,
            admin_key_string,
        })
    }

    pub fn contract_address_string(&self) -> Option<String> {
        self.contract_address_string.clone()
    }

    pub fn device_keystore_path(&self) -> Option<PathBuf> {
        self.device_keystore_path.clone()
    }

    pub fn local_ipfs_scheme(&self) -> String {
        self.local_ipfs_scheme.clone()
    }

    pub fn local_ipfs_host(&self) -> String {
        self.local_ipfs_host.clone()
    }

    pub fn local_ipfs_api_port(&self) -> String {
        self.local_ipfs_api_port.clone()
    }

    pub fn local_ipfs_gateway_port(&self) -> String {
        self.local_ipfs_gateway_port.clone()
    }

    pub fn ipfs_remote(&self) -> Option<IpfsRemote> {
        self.ipfs_remote.clone()
    }

    pub fn admin_key_string(&self) -> Option<String> {
        self.admin_key_string.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// On disk device defaults
pub struct OnDiskConfig {
    /// Root cid contract address
    root_cid_contract_address_string: Option<String>,
    /// Device keystore path
    device_keystore_path: PathBuf,
    /// Remote ipfs configuration
    ipfs_remote: Option<IpfsRemote>,
}

impl OnDiskConfig {
    /// Initialize a new default config for this device using our xdg defaults
    /// Fails if the xdg home directory already exists
    /// TODO: password workflow
    /// TODO: config detection and validation
    /// TODO: feature gaurd tracing
    pub fn init() -> Result<(), ConfigError> {
        let xdg_path = xdg_config_home()?;
        let config_path = xdg_path.join(DEFAULT_CONFIG_NAME);
        let device_keystore_path = xdg_path.join(DEFAULT_DEVICE_KEYSTORE_NAME);

        // Check if the xdg home directory exists. If not then go ahead and initialize everything
        if !xdg_path.exists() {
            create_dir_all(&xdg_path)?;

            let mut rng = rand::thread_rng();
            // Create a new keystore
            let _wallet = Wallet::new_keystore(
                &xdg_path,
                // TODO: password workflow
                &mut rng,
                "",
                Some(DEFAULT_DEVICE_KEYSTORE_NAME),
            )?;

            let config = Self {
                root_cid_contract_address_string: None,
                device_keystore_path,
                ipfs_remote: None,
            };

            // Serialize the config
            let config_json = serde_json::to_string(&config)?;
            // Write the config to disk
            let mut config_file = File::create(config_path)?;
            config_file.write_all(config_json.as_bytes())?;
        }
        // If the xdg home directory does exist, check how far we can proceed.
        else {
            tracing::info!("Xdg home directory already exists: {:?}", xdg_path);
            tracing::error!("Stopping execution. Inspect the directory and try again.");
            return Err(ConfigError::XdgPathAlreadyExists(xdg_path));
        }
        Ok(())
    }

    /// Load the config for this device using our xdg defaults
    /// Fails if the xdg home directory does not exist
    pub fn load() -> Result<Self, ConfigError> {
        let xdg_path = xdg_config_home()?;
        let config_path = xdg_path.join(DEFAULT_CONFIG_NAME);

        let config = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config)?;

        Ok(config)
    }

    pub fn root_cid_contract_address_string(&self) -> Option<String> {
        self.root_cid_contract_address_string.clone()
    }

    pub fn device_keystore_path(&self) -> PathBuf {
        self.device_keystore_path.clone()
    }

    pub fn ipfs_remote(&self) -> Option<IpfsRemote> {
        self.ipfs_remote.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("xdg home path already exists: {0}")]
    XdgPathAlreadyExists(PathBuf),
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("failed to create directory: {0}")]
    CreateDirError(#[from] std::io::Error),
    #[error("failed wallet operation: {0}")]
    KeystoreError(#[from] ethers::signers::WalletError),
    #[error("failed to parse config: {0}")]
    ConfigParseError(#[from] serde_json::Error),
    #[error("Missing config file: {0}")]
    MissingConfigFile(PathBuf),
    #[error("Missing dot path: {0}")]
    MissingDotPath(PathBuf),
    #[error("Invalid Address: {0}")]
    InvalidAddress(String),
}

/// Grab config path
pub fn xdg_config_home() -> Result<PathBuf, ConfigError> {
    // Construct
    let path = PathBuf::from(XDG_PATH.replace(
        "~",
        &env::var("HOME").map_err(|_| ConfigError::MissingEnvVar("HOME".to_string()))?,
    ));
    // Return
    Ok(path)
}
