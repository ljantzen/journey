use journey::config::Config;
use journey::config::VaultConfig;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_set_default_vault() {
    let mut config = Config::new();
    
    // Add some vaults
    let vault1 = VaultConfig {
        name: "vault1".to_string(),
        path: PathBuf::from("/path1"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    let vault2 = VaultConfig {
        name: "vault2".to_string(),
        path: PathBuf::from("/path2"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    config.add_vault(vault1);
    config.add_vault(vault2);
    
    // Initially no default vault
    assert!(config.default_vault.is_none());
    
    // Set default vault
    assert!(config.set_default_vault("vault1").is_ok());
    assert_eq!(config.default_vault, Some("vault1".to_string()));
    
    // Try to set non-existent vault
    assert!(config.set_default_vault("nonexistent").is_err());
    
    // Change default vault
    assert!(config.set_default_vault("vault2").is_ok());
    assert_eq!(config.default_vault, Some("vault2".to_string()));
}

#[test]
fn test_get_default_vault() {
    let mut config = Config::new();
    
    // Add a vault
    let vault = VaultConfig {
        name: "test_vault".to_string(),
        path: PathBuf::from("/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    config.add_vault(vault);
    
    // No default set - should return first vault
    let default = config.get_default_vault();
    assert!(default.is_some());
    assert_eq!(default.unwrap().name, "test_vault");
    
    // Set default vault
    config.set_default_vault("test_vault").unwrap();
    let default = config.get_default_vault();
    assert!(default.is_some());
    assert_eq!(default.unwrap().name, "test_vault");
}

#[test]
fn test_clear_default_vault() {
    let mut config = Config::new();
    
    let vault = VaultConfig {
        name: "test_vault".to_string(),
        path: PathBuf::from("/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    config.add_vault(vault);
    config.set_default_vault("test_vault").unwrap();
    
    assert_eq!(config.default_vault, Some("test_vault".to_string()));
    
    config.clear_default_vault();
    assert!(config.default_vault.is_none());
}

#[test]
fn test_default_vault_fallback() {
    let mut config = Config::new();
    
    // No vaults - should return None
    assert!(config.get_default_vault().is_none());
    
    // Add a vault
    let vault = VaultConfig {
        name: "test_vault".to_string(),
        path: PathBuf::from("/test"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
    };
    
    config.add_vault(vault);
    
    // No default set - should return first vault
    let default = config.get_default_vault();
    assert!(default.is_some());
    assert_eq!(default.unwrap().name, "test_vault");
    
    // Set default to non-existent vault - should return None
    config.default_vault = Some("nonexistent".to_string());
    assert!(config.get_default_vault().is_none());
}
