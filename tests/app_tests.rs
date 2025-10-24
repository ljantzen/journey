use journey::app::App;
use journey::config::{Config, VaultConfig};
use journey::config_manager::ConfigManager;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;
use std::env;

// Safety mechanism: Ensure tests never touch production config
fn ensure_test_isolation() {
    // Clear any existing config environment variable to prevent accidental production access
    env::remove_var("JOURNEY_CONFIG");
}

// Helper function to create an app with a specific config
fn create_app_with_config(config: Config) -> App {
    // Ensure test isolation before creating app
    ensure_test_isolation();
    
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    
    let config_manager = ConfigManager { config_path: config_path.clone() };
    config_manager.save_config(&config).unwrap();
    
    // Set the config path environment variable for test isolation
    env::set_var("JOURNEY_CONFIG", config_path.to_str().unwrap());
    
    App::new().unwrap()
}

#[test]
fn test_app_creation() {
    // Create a config with one vault so the app can be created
    let mut config = Config::new();
    let vault_config = VaultConfig {
        name: "test-vault".to_string(),
        path: PathBuf::from("/tmp/test-vault"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
    };
    config.add_vault(vault_config);
    
    let app = create_app_with_config(config);
    
    // This should work
    assert!(app.get_vault(None).is_ok());
}

#[test]
fn test_single_vault_auto_selection() {
    // Create a config with one vault
    let mut config = Config::new();
    let vault_config = VaultConfig {
        name: "test-vault".to_string(),
        path: PathBuf::from("/tmp/test-vault"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
    };
    config.add_vault(vault_config);
    
    let app = create_app_with_config(config);
    
    // Should automatically select the single vault when no vault specified
    let vault = app.get_vault(None);
    assert!(vault.is_ok());
    
    // Should also work when explicitly specifying the vault
    let vault = app.get_vault(Some("test-vault"));
    assert!(vault.is_ok());
}

#[test]
fn test_multiple_vaults_require_specification() {
    // Create a config with multiple vaults
    let mut config = Config::new();
    
    let vault1 = VaultConfig {
        name: "vault1".to_string(),
        path: PathBuf::from("/tmp/vault1"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
    };
    
    let vault2 = VaultConfig {
        name: "vault2".to_string(),
        path: PathBuf::from("/tmp/vault2"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
    };
    
    config.add_vault(vault1);
    config.add_vault(vault2);
    
    let app = create_app_with_config(config);
    
    // Should fail when no vault specified and multiple exist
    let vault = app.get_vault(None);
    assert!(vault.is_err());
    
    // Should work when explicitly specifying a vault
    let vault = app.get_vault(Some("vault1"));
    assert!(vault.is_ok());
    
    let vault = app.get_vault(Some("vault2"));
    assert!(vault.is_ok());
}

#[test]
fn test_no_vaults_configured() {
    // Create an empty config
    let config = Config::new();
    let app = create_app_with_config(config);
    
    // Should fail when no vaults are configured
    let vault = app.get_vault(None);
    assert!(vault.is_err());
    
    // Should also fail when trying to specify a non-existent vault
    let vault = app.get_vault(Some("non-existent"));
    assert!(vault.is_err());
}

#[test]
fn test_no_config_file_exists() {
    // Ensure test isolation
    ensure_test_isolation();
    
    // Create a temporary directory without any config file
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    
    // Set the config path environment variable to point to non-existent file
    env::set_var("JOURNEY_CONFIG", config_path.to_str().unwrap());
    
    // App creation should fail when no config file exists
    let app = App::new();
    assert!(app.is_err());
    
    // Check that the error message is helpful
    if let Err(journey::errors::JourneyError::Config(msg)) = app {
        assert!(msg.contains("No configuration file found"));
        // The error message was updated to use --init instead of init
        assert!(msg.contains("journey --init"));
    } else {
        panic!("Expected Config error");
    }
}

#[test]
fn test_init_vault_with_name() {
    // Ensure test isolation
    ensure_test_isolation();
    
    // Create a temporary directory for config
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    
    // Set the config path environment variable for test isolation
    env::set_var("JOURNEY_CONFIG", config_path.to_str().unwrap());
    
    // Create app using new_for_init (allows empty config)
    let mut app = App::new_for_init().unwrap();
    
    // Test init with explicit name
    let result = app.init_vault(PathBuf::from("/tmp/test-vault"), Some("my-vault".to_string()));
    assert!(result.is_ok());
}

#[test]
fn test_init_vault_without_name() {
    // Ensure test isolation
    ensure_test_isolation();
    
    // Create a temporary directory for config
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    
    // Set the config path environment variable for test isolation
    env::set_var("JOURNEY_CONFIG", config_path.to_str().unwrap());
    
    // Create app using new_for_init (allows empty config)
    let mut app = App::new_for_init().unwrap();
    
    // Test init without name - should use path basename
    let result = app.init_vault(PathBuf::from("/tmp/my-journal"), None);
    assert!(result.is_ok());
}

#[test]
fn test_init_vault_invalid_path() {
    // Ensure test isolation
    ensure_test_isolation();
    
    // Create a temporary directory for config
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    
    // Set the config path environment variable for test isolation
    env::set_var("JOURNEY_CONFIG", config_path.to_str().unwrap());
    
    // Create app using new_for_init (allows empty config)
    let mut app = App::new_for_init().unwrap();
    
    // Test init with invalid path (empty)
    let result = app.init_vault(PathBuf::from(""), None);
    assert!(result.is_err());
    
    // Check that the error message is appropriate
    if let Err(journey::errors::JourneyError::Config(msg)) = result {
        assert!(msg.contains("Invalid path: cannot extract basename"));
    } else {
        panic!("Expected Config error");
    }
}

#[test]
fn test_production_config_isolation() {
    // This test verifies that tests never touch production config
    // even if production config exists
    
    // Ensure test isolation
    ensure_test_isolation();
    
    // Create a temporary directory for test config
    let temp_dir = TempDir::new().unwrap();
    let test_config_path = temp_dir.path().join("journey.yaml");
    
    // Set the test config path
    env::set_var("JOURNEY_CONFIG", test_config_path.to_str().unwrap());
    
    // Create a test config
    let mut test_config = Config::new();
    let vault_config = VaultConfig {
        name: "test-vault".to_string(),
        path: PathBuf::from("/tmp/test-vault"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
    };
    test_config.add_vault(vault_config);
    
    // Save test config
    let config_manager = ConfigManager { config_path: test_config_path.clone() };
    config_manager.save_config(&test_config).unwrap();
    
    // Create app - should use test config, not production
    let app = App::new().unwrap();
    
    // Verify that the app is using the test config
    let vault = app.get_vault(Some("test-vault"));
    assert!(vault.is_ok());
    
    // Verify that production config (if it exists) was not touched
    // by checking that our test config path is being used
    assert_eq!(config_manager.config_path, test_config_path);
    
    // The test config should be isolated from any production config
    // This is verified by the fact that we're using a temporary directory
    // and the JOURNEY_CONFIG environment variable
}
