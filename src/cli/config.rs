use std::collections::HashMap;
use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cid::Cid;
use ethers::signers::Wallet;
use serde::{Deserialize, Serialize};

use crate::ipfs::{IpfsClient, IpfsError, IpfsRemote};
use crate::root_cid::{
    EthClient as RootCidClient, EthClientError as RootCidClientError, EthRemote,
};

use super::args::{Args, ConfigureCreateSubcommand, ConfigureSetSubcommand, ConfigureSubcommand};

const XDG_PATH: &str = "~/.config/dor-store";
const DEFAULT_CONFIG_NAME: &str = "defaults.json";
const DEAULT_ETH_REMOTE_CONFIG_DIR: &str = "eth";
const DEFAULT_IPFS_REMOTE_CONFIG_DIR: &str = "ipfs";
const DEFAULT_DEVICE_KEYSTORE_NAME: &str = "device-keystore.json";
pub const DEFAULT_LOCAL_DOT_DIR: &str = ".fs";
pub const DEFAULT_LOCAL_DOT_CHANGELOG: &str = "change_log.json";
pub const DEFAULT_LOCAL_DOT_DORFS: &str = "dorfs.json";
pub const DEFAULT_LOCAL_DOT_ROOTCID: &str = "root_cid";

pub fn handle_config_subcommand(
    _config: &Config,
    subcommand: ConfigureSubcommand,
) -> Result<(), ConfigError> {
    match subcommand {
        ConfigureSubcommand::Create { subcommand } => match subcommand {
            ConfigureCreateSubcommand::Eth {
                alias,
                rpc,
                address,
                chain_id,
            } => {
                let eth_remote = EthRemote {
                    rpc,
                    address,
                    chain_id,
                };
                let _eth_client = RootCidClient::try_from(eth_remote.clone())?;
                OnDiskConfig::create_eth_remote(alias, eth_remote)?;
            }
            ConfigureCreateSubcommand::Ipfs {
                alias,
                url,
                gateway_url,
            } => {
                let ipfs_remote = IpfsRemote { url, gateway_url };
                let _ipfs_client = IpfsClient::try_from(ipfs_remote.clone())?;
                OnDiskConfig::create_ipfs_remote(alias, ipfs_remote)?;
            }
        },
        ConfigureSubcommand::Set { subcommand } => match subcommand {
            ConfigureSetSubcommand::Eth { alias } => {
                let mut on_disk_config = OnDiskConfig::load()?;
                on_disk_config.set_eth_remote_alias(alias)?;
            }
            ConfigureSetSubcommand::Ipfs { alias } => {
                let mut on_disk_config = OnDiskConfig::load()?;
                on_disk_config.set_ipfs_remote_alias(alias)?;
            }
        },
        ConfigureSubcommand::Ls => {
            let on_disk_config = OnDiskConfig::load()?;
            let eth_remotes = on_disk_config.eth_remotes()?;
            let ipfs_remote = on_disk_config.ipfs_remotes()?;
            println!("Eth remotes:");
            for (alias, eth_remote) in eth_remotes.iter() {
                println!("{}: {:?}", alias, eth_remote);
            }
            println!("Ipfs remotes:");
            for (alias, ipfs_remote) in ipfs_remote.iter() {
                println!("{}: {:?}", alias, ipfs_remote);
            }
        }
        ConfigureSubcommand::Show => {
            let on_disk_config = OnDiskConfig::load()?;
            let eth_remote = on_disk_config.eth_remote()?;
            let ipfs_remote = on_disk_config.ipfs_remote()?;
            println!("Eth remote: {:?}", eth_remote);
            println!("Ipfs remote: {:?}", ipfs_remote);
        }
    }
    Ok(())
}

// TODO: ipfs configuration
#[derive(Debug)]
pub struct Config {
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

    /// Remote configuration for an eth node
    eth_remote: Option<EthRemote>,

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
        // This is wierd but makes the interface clean
        // There's a weird responsibility boundary crossing here but
        if args.command.clone() == super::args::Command::Init {
            OnDiskConfig::init()?;
            std::process::exit(0);
        }

        // (maybe) Load the on disk config
        let maybe_on_disk_config = match OnDiskConfig::load() {
            Ok(on_disk_config) => Some(on_disk_config),
            Err(e) => {
                tracing::error!("Failed to load on disk config: {:?}", e);
                return Err(e);
            }
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

        let ipfs_remote = match maybe_on_disk_config.clone() {
            Some(on_disk_config) => match on_disk_config.ipfs_remote() {
                Ok(ipfs_remote) => Some(ipfs_remote),
                Err(ConfigError::MissingIpfsRemoteAlias) => None,
                Err(e) => {
                    return Err(e);
                }
            },
            None => None,
        };

        let eth_remote = match maybe_on_disk_config.clone() {
            Some(on_disk_config) => match on_disk_config.eth_remote() {
                Ok(eth_remote) => Some(eth_remote),
                Err(ConfigError::MissingEthRemoteAlias) => None,
                Err(e) => {
                    return Err(e);
                }
            },
            None => None,
        };

        let admin_key_string = args.admin_key.clone();

        Ok(Self {
            device_keystore_path,
            local_ipfs_scheme,
            local_ipfs_host,
            local_ipfs_api_port,
            local_ipfs_gateway_port,
            eth_remote,
            ipfs_remote,
            admin_key_string,
        })
    }

    pub fn device_keystore_path(&self) -> Option<PathBuf> {
        self.device_keystore_path.clone()
    }

    pub fn local_ipfs_remote(&self) -> IpfsRemote {
        IpfsRemote {
            url: format!(
                "{}://{}:{}",
                self.local_ipfs_scheme, self.local_ipfs_host, self.local_ipfs_api_port
            ),
            gateway_url: Some(format!(
                "{}://{}:{}",
                self.local_ipfs_scheme, self.local_ipfs_host, self.local_ipfs_gateway_port
            )),
        }
    }

    pub fn ipfs_remote(&self) -> Option<IpfsRemote> {
        self.ipfs_remote.clone()
    }

    pub fn eth_remote(&self) -> Option<EthRemote> {
        self.eth_remote.clone()
    }

    pub fn admin_key_string(&self) -> Option<String> {
        self.admin_key_string.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// On disk device defaults
pub struct OnDiskConfig {
    /// Device keystore path
    device_keystore_path: PathBuf,
    /// root cid
    root_cid: Option<Cid>,
    /// Remote eth configuration
    eth_remote_alias: Option<String>,
    /// Remote ipfs configuration
    ipfs_remote_alias: Option<String>,
}

impl OnDiskConfig {
    /// Initialize a new default config for this device using our xdg defaults
    /// Fails if the xdg home directory already exists
    /// TODO: password workflow
    /// TODO: config detection and validation
    /// TODO: feature gaurd tracing
    pub fn init() -> Result<(), ConfigError> {
        let xdg_path = xdg_config_home()?;
        let eth_remote_path = xdg_path.join(DEAULT_ETH_REMOTE_CONFIG_DIR);
        let ipfs_remote_path = xdg_path.join(DEFAULT_IPFS_REMOTE_CONFIG_DIR);
        let device_keystore_path = xdg_path.join(DEFAULT_DEVICE_KEYSTORE_NAME);

        // Check if the xdg home directory exists. If not then go ahead and initialize everything
        if !xdg_path.exists() {
            create_dir_all(&xdg_path)?;
            create_dir_all(eth_remote_path)?;
            create_dir_all(ipfs_remote_path)?;

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
                device_keystore_path,
                root_cid: None,
                ipfs_remote_alias: None,
                eth_remote_alias: None,
            };
            config.save()?;
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

    /// Load the Eth remote configuration using the alias
    pub fn eth_remote(&self) -> Result<EthRemote, ConfigError> {
        let xdg_path = xdg_config_home()?;
        let eth_remote_dir_path = xdg_path.join(DEAULT_ETH_REMOTE_CONFIG_DIR);
        let eth_remote_config_path = match self.eth_remote_alias.clone() {
            Some(eth_remote_alias) => eth_remote_dir_path.join(eth_remote_alias),
            None => {
                return Err(ConfigError::MissingEthRemoteAlias);
            }
        };
        let eth_remote_config = std::fs::read_to_string(eth_remote_config_path)?;
        let eth_remote: EthRemote = serde_json::from_str(&eth_remote_config)?;

        Ok(eth_remote)
    }

    /// Get a map of all the eth remotes by alias
    pub fn eth_remotes(&self) -> Result<HashMap<String, EthRemote>, ConfigError> {
        let xdg_path = xdg_config_home()?;
        let eth_remote_dir_path = xdg_path.join(DEAULT_ETH_REMOTE_CONFIG_DIR);
        let mut eth_remotes = HashMap::new();
        for entry in std::fs::read_dir(eth_remote_dir_path)? {
            let entry = entry?;
            let eth_remote_config = std::fs::read_to_string(entry.path())?;
            let eth_remote: EthRemote = serde_json::from_str(&eth_remote_config)?;
            eth_remotes.insert(entry.file_name().into_string().unwrap(), eth_remote);
        }
        Ok(eth_remotes)
    }

    /// Create a new eth remote configuration using the alias
    pub fn create_eth_remote(
        eth_remote_alias: String,
        eth_remote: EthRemote,
    ) -> Result<(), ConfigError> {
        let xdg_path = xdg_config_home()?;
        let eth_remote_dir_path = xdg_path.join(DEAULT_ETH_REMOTE_CONFIG_DIR);
        let eth_remote_config_path = eth_remote_dir_path.join(eth_remote_alias.clone());

        // Serialize the config
        let eth_remote_config = serde_json::to_string(&eth_remote)?;
        // Write the config to disk
        let mut eth_remote_config_file = File::create(eth_remote_config_path)?;
        eth_remote_config_file.write_all(eth_remote_config.as_bytes())?;

        Ok(())
    }

    /// Set the eth remote alias
    pub fn set_eth_remote_alias(&mut self, eth_remote_alias: String) -> Result<(), ConfigError> {
        self.eth_remote_alias = Some(eth_remote_alias);
        self.save()
    }

    /// Load the Ipfs remote configuration using the alias
    pub fn ipfs_remote(&self) -> Result<IpfsRemote, ConfigError> {
        let xdg_path = xdg_config_home()?;
        let ipfs_remote_dir_path = xdg_path.join(DEFAULT_IPFS_REMOTE_CONFIG_DIR);
        let ipfs_remote_config_path = match self.ipfs_remote_alias.clone() {
            Some(ipfs_remote_alias) => ipfs_remote_dir_path.join(ipfs_remote_alias),
            None => {
                return Err(ConfigError::MissingIpfsRemoteAlias);
            }
        };
        let ipfs_remote_config = std::fs::read_to_string(ipfs_remote_config_path)?;
        let ipfs_remote: IpfsRemote = serde_json::from_str(&ipfs_remote_config)?;

        Ok(ipfs_remote)
    }

    /// Get a map of all the ipfs remotes by alias
    pub fn ipfs_remotes(&self) -> Result<HashMap<String, IpfsRemote>, ConfigError> {
        let xdg_path = xdg_config_home()?;
        let ipfs_remote_dir_path = xdg_path.join(DEFAULT_IPFS_REMOTE_CONFIG_DIR);
        let mut ipfs_remotes = HashMap::new();
        for entry in std::fs::read_dir(ipfs_remote_dir_path)? {
            let entry = entry?;
            let ipfs_remote_config = std::fs::read_to_string(entry.path())?;
            let ipfs_remote: IpfsRemote = serde_json::from_str(&ipfs_remote_config)?;
            ipfs_remotes.insert(entry.file_name().into_string().unwrap(), ipfs_remote);
        }
        Ok(ipfs_remotes)
    }

    /// Create a new ipfs remote configuration using the alias
    pub fn create_ipfs_remote(
        ipfs_remote_alias: String,
        ipfs_remote: IpfsRemote,
    ) -> Result<(), ConfigError> {
        let xdg_path = xdg_config_home()?;
        let ipfs_remote_dir_path = xdg_path.join(DEFAULT_IPFS_REMOTE_CONFIG_DIR);
        let ipfs_remote_config_path = ipfs_remote_dir_path.join(ipfs_remote_alias.clone());

        // Serialize the config
        let ipfs_remote_config = serde_json::to_string(&ipfs_remote)?;
        // Write the config to disk
        let mut ipfs_remote_config_file = File::create(ipfs_remote_config_path)?;
        ipfs_remote_config_file.write_all(ipfs_remote_config.as_bytes())?;

        Ok(())
    }

    /// Set the ipfs remote alias
    pub fn set_ipfs_remote_alias(&mut self, ipfs_remote_alias: String) -> Result<(), ConfigError> {
        self.ipfs_remote_alias = Some(ipfs_remote_alias);
        self.save()
    }

    pub fn device_keystore_path(&self) -> PathBuf {
        self.device_keystore_path.clone()
    }

    fn save(&self) -> Result<(), ConfigError> {
        let xdg_path = xdg_config_home()?;
        let config_path = xdg_path.join(DEFAULT_CONFIG_NAME);

        // Serialize the config
        let config_json = serde_json::to_string(&self)?;
        // Write the config to disk
        let mut config_file = File::create(config_path)?;
        config_file.write_all(config_json.as_bytes())?;

        Ok(())
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
    #[error("Missing dot path: {0}")]
    MissingDotPath(PathBuf),
    #[error("Missing root cid")]
    MissingRootCid,
    #[error("No configured eth remote alias")]
    MissingEthRemoteAlias,
    #[error("No configured ipfs remote alias")]
    MissingIpfsRemoteAlias,
    #[error("Invalid eth remote: {0}")]
    InvalidEthRemote(#[from] RootCidClientError),
    #[error("Invalid ipfs remote: {0}")]
    InvalidIpfsRemote(#[from] IpfsError),
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
