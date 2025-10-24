use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vaults: HashMap<String, VaultConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub name: String,
    pub path: PathBuf,
    pub locale: String,
    pub phrases: HashMap<String, String>,
    pub section_name: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vaults: HashMap::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vault(&mut self, vault: VaultConfig) {
        self.vaults.insert(vault.name.clone(), vault);
    }

    pub fn get_vault(&self, name: &str) -> Option<&VaultConfig> {
        self.vaults.get(name)
    }

    pub fn get_default_vault(&self) -> Option<&VaultConfig> {
        // For now, return the first vault if any exist
        self.vaults.values().next()
    }
}

