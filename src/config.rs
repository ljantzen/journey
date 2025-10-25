use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;
use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vaults: HashMap<String, VaultConfig>,
    pub default_vault: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub name: String,
    #[serde(deserialize_with = "deserialize_path_with_tilde_expansion")]
    pub path: PathBuf,
    pub locale: String,
    pub phrases: HashMap<String, String>,
    pub section_name: Option<String>,
    pub date_format: Option<String>,
    #[serde(deserialize_with = "deserialize_template_file_with_expansion")]
    pub template_file: Option<String>,
    pub file_path_format: Option<String>,
}

/// Custom deserializer for PathBuf that expands tildes
fn deserialize_path_with_tilde_expansion<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let path_str = String::deserialize(deserializer)?;
    Ok(expand_tilde(&path_str))
}

/// Custom deserializer for template_file that expands tildes and Windows env vars
fn deserialize_template_file_with_expansion<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let template_file: Option<String> = Option::deserialize(deserializer)?;
    Ok(template_file.map(|path| expand_tilde(&path).to_string_lossy().to_string()))
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vaults: HashMap::new(),
            default_vault: None,
        }
    }
}

/// Expand tilde (~) and Windows environment variables (%VAR%) in a path string
/// Cross-platform implementation that handles:
/// - Unix/Linux/macOS: ~/path -> $HOME/path
/// - Windows: ~/path -> %USERPROFILE%/path
/// - Windows: %USERPROFILE%\path -> C:\Users\username\path
/// - Windows: %APPDATA%\path -> C:\Users\username\AppData\Roaming\path
fn expand_tilde(path: &str) -> PathBuf {
    // First check for tilde expansion
    if path.starts_with("~/") {
        // Try to get home directory from environment variables
        let home_dir = if cfg!(windows) {
            // On Windows, try USERPROFILE first, then HOME
            env::var("USERPROFILE")
                .or_else(|_| env::var("HOME"))
                .ok()
        } else {
            // On Unix-like systems, use HOME
            env::var("HOME").ok()
        };
        
        if let Some(home) = home_dir {
            PathBuf::from(home).join(&path[2..])
        } else {
            // If no home directory found, return the path as-is
            PathBuf::from(path)
        }
    } else if path == "~" {
        // Just the tilde by itself
        let home_dir = if cfg!(windows) {
            env::var("USERPROFILE")
                .or_else(|_| env::var("HOME"))
                .ok()
        } else {
            env::var("HOME").ok()
        };
        
        if let Some(home) = home_dir {
            PathBuf::from(home)
        } else {
            PathBuf::from(path)
        }
    } else {
        // Check for Windows-style environment variable expansion
        expand_windows_env_vars(path)
    }
}

/// Expand Windows-style environment variables (%VAR%) in a path string
fn expand_windows_env_vars(path: &str) -> PathBuf {
    if !cfg!(windows) {
        // On non-Windows systems, return the path as-is
        return PathBuf::from(path);
    }
    
    let mut result = path.to_string();
    let mut start = 0;
    
    // Find and replace %VAR% patterns
    while let Some(var_start) = result[start..].find('%') {
        let actual_start = start + var_start;
        let var_start_pos = actual_start + 1; // Skip the %
        
        if let Some(var_end) = result[var_start_pos..].find('%') {
            let var_end_pos = var_start_pos + var_end;
            let var_name = &result[var_start_pos..var_end_pos];
            
            // Try to get the environment variable
            if let Ok(var_value) = env::var(var_name) {
                // Replace %VAR% with the actual value
                result.replace_range(actual_start..=var_end_pos, &var_value);
                start = actual_start + var_value.len();
            } else {
                // Variable not found, skip this occurrence
                start = var_end_pos + 1;
            }
        } else {
            // No closing %, break
            break;
        }
    }
    
    PathBuf::from(result)
}

impl VaultConfig {
    /// Create a new VaultConfig with tilde expansion applied to the path
    pub fn new(name: String, path: String, locale: String) -> Self {
        Self {
            name,
            path: expand_tilde(&path),
            locale,
            phrases: HashMap::new(),
            section_name: None,
            date_format: None,
            template_file: None,
            file_path_format: None,
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
        if let Some(default_name) = &self.default_vault {
            self.vaults.get(default_name)
        } else {
            // Fallback to first vault if no default is set
            self.vaults.values().next()
        }
    }

    pub fn set_default_vault(&mut self, vault_name: &str) -> Result<(), String> {
        if self.vaults.contains_key(vault_name) {
            self.default_vault = Some(vault_name.to_string());
            Ok(())
        } else {
            Err(format!("Vault '{}' not found", vault_name))
        }
    }

    pub fn clear_default_vault(&mut self) {
        self.default_vault = None;
    }
}

