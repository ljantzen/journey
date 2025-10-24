use journey::config::{Config, VaultConfig};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_config_new() {
    let config = Config::new();
    assert!(config.vaults.is_empty());
}

#[test]
fn test_config_add_vault() {
    let mut config = Config::new();
    let vault = VaultConfig {
        name: "test".to_string(),
        path: PathBuf::from("/tmp/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    config.add_vault(vault);
    assert_eq!(config.vaults.len(), 1);
    assert!(config.vaults.contains_key("test"));
}

#[test]
fn test_config_get_vault() {
    let mut config = Config::new();
    let vault = VaultConfig {
        name: "test".to_string(),
        path: PathBuf::from("/tmp/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    config.add_vault(vault);
    
    let retrieved = config.get_vault("test");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "test");
    
    let not_found = config.get_vault("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_config_get_default_vault() {
    let mut config = Config::new();
    
    // No vaults
    assert!(config.get_default_vault().is_none());
    
    // Add a vault
    let vault = VaultConfig {
        name: "test".to_string(),
        path: PathBuf::from("/tmp/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    config.add_vault(vault);
    let default = config.get_default_vault();
    assert!(default.is_some());
    assert_eq!(default.unwrap().name, "test");
}

#[test]
fn test_vault_config_creation() {
    let vault = VaultConfig {
        name: "test".to_string(),
        path: PathBuf::from("/tmp/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    assert_eq!(vault.name, "test");
    assert_eq!(vault.path, PathBuf::from("/tmp/test"));
    assert_eq!(vault.locale, "en-US");
    assert_eq!(vault.section_name, Some("Daily Notes".to_string()));
}
