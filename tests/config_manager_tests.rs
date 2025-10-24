use journey::config_manager::ConfigManager;
use journey::config::{Config, VaultConfig};
use std::collections::HashMap;
use tempfile::TempDir;
use std::path::PathBuf;

fn create_test_config_manager() -> (ConfigManager, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    let config_manager = ConfigManager { config_path };
    (config_manager, temp_dir)
}

#[test]
fn test_config_manager_creation() {
    let (config_manager, _temp_dir) = create_test_config_manager();
    assert!(!config_manager.config_exists());
}

#[test]
fn test_load_config_nonexistent() {
    let (config_manager, _temp_dir) = create_test_config_manager();
    let config = config_manager.load_config().unwrap();
    assert!(config.vaults.is_empty());
}

#[test]
fn test_save_and_load_config() {
    let (config_manager, _temp_dir) = create_test_config_manager();
    
    let mut config = Config::new();
    let vault = VaultConfig {
        name: "test".to_string(),
        path: PathBuf::from("/tmp/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
    };
    config.add_vault(vault);
    
    config_manager.save_config(&config).unwrap();
    assert!(config_manager.config_exists());
    
    let loaded_config = config_manager.load_config().unwrap();
    assert_eq!(loaded_config.vaults.len(), 1);
    assert!(loaded_config.vaults.contains_key("test"));
}

#[test]
fn test_config_exists() {
    let (config_manager, _temp_dir) = create_test_config_manager();
    
    assert!(!config_manager.config_exists());
    
    let config = Config::new();
    config_manager.save_config(&config).unwrap();
    
    assert!(config_manager.config_exists());
}
