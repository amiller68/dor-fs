use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use super::{xdg_config_home, ConfigError, DEFAULT_CONFIG_NAME};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// On Disk Cli Defaults
pub struct OnDiskDefault {
    /// Set device alias
    device_alias: Option<String>,
}

impl OnDiskDefault {
    /// Return the on disk defaults
    /// Initializes a config + xdg home dir of none exists
    pub fn load() -> Result<Self, ConfigError> {
        let xdg_path = xdg_config_home()?;

        // Check if the xdg home directory exists.
        // If not, create one and return the default.
        if !xdg_path.exists() {
            create_dir_all(&xdg_path)?;
            let config = Self::default();
            config.save()?;
            Ok(config)
        }

        // Otherwise load the config
        let config_path = xdg_path.join(DEFAULT_CONFIG_NAME);
        let config = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config)?;

        Ok(config)
    }

    /// Update what default device to use
    /// Saves the updated config to disk
    pub fn set_device_alias(&mut self, alias: String) -> Result<(), ConfigError> {
        self.device_alias = Some(String);
        self.save()?
    }

    /// Read the set alias, if any
    pub fn device_alias(&self) -> Option<String> {
        self.device_alias.clone()
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
