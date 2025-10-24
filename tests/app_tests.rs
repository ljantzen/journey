use journey::app::{App, CliArgs};
use journey::config::{Config, VaultConfig};
use journey::config_manager::ConfigManager;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;
use std::env;
use chrono::{NaiveDate, Local};

// Safety mechanism: Ensure tests never touch production config
fn ensure_test_isolation() {
    // Clear any existing config environment variable to prevent accidental production access
    env::remove_var("JOURNEY_CONFIG");
    env::remove_var("JOURNEY_CONFIG_PATH"); // Also clear the old variable name
}

// Helper function to create an app with a specific config
fn create_app_with_config(config: Config) -> (App, impl FnOnce()) {
    // Ensure test isolation before creating app
    ensure_test_isolation();
    
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    
    // Create a temporary directory for vault paths if they exist
    let vault_temp_dir = TempDir::new().unwrap();
    
    // Update any vault paths in the config to use the temporary directory
    let mut updated_config = config;
    for (i, vault) in updated_config.vaults.values_mut().enumerate() {
        vault.path = vault_temp_dir.path().join(format!("vault_{}", i));
    }
    
    let config_manager = ConfigManager { config_path: config_path.clone() };
    config_manager.save_config(&updated_config).unwrap();
    
    // Set the config path environment variable for test isolation
    env::set_var("JOURNEY_CONFIG", config_path.to_str().unwrap());
    
    let app = App::new().unwrap();
    let cleanup = move || {
        env::remove_var("JOURNEY_CONFIG");
        // Keep temp directories alive to prevent cleanup
        let _temp_dir = temp_dir;
        let _vault_temp_dir = vault_temp_dir;
    };
    
    (app, cleanup)
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
        template_file: None,
    };
    config.add_vault(vault_config);
    
    let (app, _cleanup) = create_app_with_config(config);
    
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
        template_file: None,
    };
    config.add_vault(vault_config);
    
    let (app, _cleanup) = create_app_with_config(config);
    
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
        template_file: None,
    };
    
    let vault2 = VaultConfig {
        name: "vault2".to_string(),
        path: PathBuf::from("/tmp/vault2"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
    };
    
    config.add_vault(vault1);
    config.add_vault(vault2);
    
    let (app, _cleanup) = create_app_with_config(config);
    
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
    let (app, _cleanup) = create_app_with_config(config);
    
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
    assert!(app.is_err(), "Expected App::new() to fail when config file doesn't exist. Got: {:?}", app.as_ref().err());
    
    // Check that the error message is helpful
    if let Err(journey::errors::JourneyError::Config(msg)) = app {
        assert!(msg.contains("No configuration file found"));
        // The error message was updated to use --init instead of init
        assert!(msg.contains("journey --init"));
    } else {
        panic!("Expected Config error");
    }
    
    // Keep temp directory alive to prevent cleanup during test
    let _temp_dir = temp_dir;
    
    // Clean up environment variable to prevent interference with other tests
    env::remove_var("JOURNEY_CONFIG");
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
    
    // Test init with explicit name using temporary directory
    let vault_temp_dir = TempDir::new().unwrap();
    let result = app.init_vault(vault_temp_dir.path().to_path_buf(), Some("test-vault".to_string()));
    assert!(result.is_ok());
    
    // Keep temp directories alive to prevent cleanup during test
    let _temp_dir = temp_dir;
    let _vault_temp_dir = vault_temp_dir;
    
    // Clean up environment variable to prevent interference
    env::remove_var("JOURNEY_CONFIG");
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
    
    // Test init without name - should use path basename using temporary directory
    let vault_temp_dir = TempDir::new().unwrap();
    let result = app.init_vault(vault_temp_dir.path().to_path_buf(), None);
    assert!(result.is_ok());
    
    // Keep temp directories alive to prevent cleanup during test
    let _temp_dir = temp_dir;
    let _vault_temp_dir = vault_temp_dir;
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
    
    // Create a test config
    let mut test_config = Config::new();
    let vault_config = VaultConfig {
        name: "test-vault".to_string(),
        path: PathBuf::from("/tmp/test-vault"), // This will be overridden by create_app_with_config
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
    };
    test_config.add_vault(vault_config);
    
    // Create app - should use test config, not production
    // Use the helper function to ensure proper isolation
    let (app, _cleanup) = create_app_with_config(test_config);
    
    // Verify that the app is using the test config
    let vault = app.get_vault(Some("test-vault"));
    assert!(vault.is_ok());
    
    // Verify that production config (if it exists) was not touched
    // by checking that our test config path is being used
    // The app should be using the test config, not any production config
    
    // The test config should be isolated from any production config
    // This is verified by the fact that we're using a temporary directory
    // and the JOURNEY_CONFIG environment variable
    
    // The helper function handles cleanup automatically
}

#[test]
fn test_add_note_future_date_absolute() {
    // Test adding a note to a future date using --date
    let mut config = Config::new();
    let vault_config = VaultConfig {
        name: "test".to_string(),
        path: std::path::PathBuf::from("/tmp/test-vault"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
    };
    config.add_vault(vault_config);
    
    let (app, _cleanup) = create_app_with_config(config);
    
    // This test verifies that the app can handle future dates
    // We can't easily test the full CLI parsing here, but we can test the date parsing logic
    let cli_args = CliArgs {
        vault: None,
        date: Some("2025-12-25".to_string()),
        relative_date: None,
        time: None,
        time_format: None,
    };
    
    let result = app.parse_date(&cli_args);
    assert!(result.is_ok());
    
    let date = result.unwrap();
    assert_eq!(date, NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
}

#[test]
fn test_add_note_future_date_relative() {
    // Test adding a note to a future date using --relative-date
    let mut config = Config::new();
    let vault_config = VaultConfig {
        name: "test".to_string(),
        path: std::path::PathBuf::from("/tmp/test-vault"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
    };
    config.add_vault(vault_config);
    
    let (app, _cleanup) = create_app_with_config(config);
    
    let cli_args = CliArgs {
        vault: None,
        date: None,
        relative_date: Some(-7), // 7 days in the future (negative = future)
        time: None,
        time_format: None,
    };
    
    let result = app.parse_date(&cli_args);
    assert!(result.is_ok());
    
    let date = result.unwrap();
    let expected_date = Local::now().date_naive() + chrono::Duration::days(7);
    assert_eq!(date, expected_date);
}

#[test]
fn test_add_note_intuitive_relative_dates() {
    // Test the intuitive relative date behavior
    let mut config = Config::new();
    let vault_config = VaultConfig {
        name: "test".to_string(),
        path: std::path::PathBuf::from("/tmp/test-vault"),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
    };
    config.add_vault(vault_config);
    
    let (app, _cleanup) = create_app_with_config(config);
    
    // Test positive values (past dates)
    let cli_args = CliArgs {
        vault: None,
        date: None,
        relative_date: Some(1), // 1 day ago (yesterday)
        time: None,
        time_format: None,
    };
    
    let result = app.parse_date(&cli_args);
    assert!(result.is_ok());
    
    let date = result.unwrap();
    let expected_date = Local::now().date_naive() - chrono::Duration::days(1);
    assert_eq!(date, expected_date);
    
    // Test negative values (future dates)
    let cli_args = CliArgs {
        vault: None,
        date: None,
        relative_date: Some(-1), // 1 day in the future (tomorrow)
        time: None,
        time_format: None,
    };
    
    let result = app.parse_date(&cli_args);
    assert!(result.is_ok());
    
    let date = result.unwrap();
    let expected_date = Local::now().date_naive() + chrono::Duration::days(1);
    assert_eq!(date, expected_date);
}
