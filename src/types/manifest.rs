use serde::{Deserialize, Serialize};

use crate::env::APP_VERSION;

#[cfg(not(target_arch = "wasm32"))]
use crate::error::{KrondorError, KrondorResult};

use super::item::Item;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Manifest {
    pub version: String,
    pub items: Vec<Item>,
}

impl Manifest {
    pub fn item(&self, name: &str) -> Option<Item> {
        self.items.iter().find(|i| i.name() == name).cloned()
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            version: APP_VERSION.to_string(),
            items: vec![],
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Manifest {
    pub fn new_space() -> KrondorResult<()> {
        use crate::utils::fs;
        fs::init_space()?;
        Ok(())
    }
    pub fn add_item(&mut self, item: Item) {
        // Check if the item already exists
        if self.item(item.name()).is_some() {
            return;
        }
        self.items.push(item);
    }
    pub fn load() -> KrondorResult<Self> {
        use crate::utils::fs;
        let path = fs::manifest_path();
        let json = std::fs::read_to_string(path).map_err(KrondorError::Io)?;
        let manifest = serde_json::from_str::<Self>(&json).map_err(KrondorError::Serde)?;
        Ok(manifest)
    }
    pub fn save(&self) -> KrondorResult<()> {
        use crate::utils::fs;
        let path = fs::manifest_path();
        let json = serde_json::to_string_pretty(self).map_err(KrondorError::Serde)?; // TODO: Remove pretty (for now
        std::fs::write(path, json).map_err(KrondorError::Io)?;
        Ok(())
    }
}
