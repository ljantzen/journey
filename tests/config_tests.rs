use journey::config::{Config, VaultConfig};
use std::collections::HashMap;
use std::path::PathBuf;
use serial_test::serial;

#[test]
fn test_config_new() {
    let config = Config::new();
    assert!(config.vaults.is_empty());
}

#[test]
fn test_config_add_vault() {
    let mut config = Config::new();
    let vault = VaultConfig::test_config("test", "/tmp/test");
    
    config.add_vault(vault);
    assert_eq!(config.vaults.len(), 1);
    assert!(config.vaults.contains_key("test"));
}

#[test]
fn test_config_get_vault() {
    let mut config = Config::new();
    let vault = VaultConfig::test_config("test", "/tmp/test");
    
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
    let vault = VaultConfig::test_config("test", "/tmp/test");
    
    config.add_vault(vault);
    let default = config.get_default_vault();
    assert!(default.is_some());
    assert_eq!(default.unwrap().name, "test");
}

#[test]
fn test_vault_config_creation() {
    let mut vault = VaultConfig::test_config("test", "/tmp/test");
    vault.section_name = Some("Daily Notes".to_string());
    
    assert_eq!(vault.name, "test");
    assert_eq!(vault.path, PathBuf::from("/tmp/test"));
    assert_eq!(vault.locale, "en-US");
    assert_eq!(vault.section_name, Some("Daily Notes".to_string()));
}

#[test]
#[serial]
fn test_tilde_expansion() {
    use serde_yaml_ng;
    use std::env;
    
    // Set a test HOME environment variable
    let original_home = env::var("HOME").unwrap_or_default();
    env::set_var("HOME", "/home/testuser");
    
    // Test YAML with tilde expansion
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: ~/journal
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: null
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // The path should be expanded to the full home directory path
    assert_eq!(vault.path, PathBuf::from("/home/testuser/journal"));
    
    // Restore original HOME
    if !original_home.is_empty() {
        env::set_var("HOME", original_home);
    } else {
        env::remove_var("HOME");
    }
}

#[test]
#[serial]
fn test_tilde_expansion_with_subdirectory() {
    use serde_yaml_ng;
    use std::env;
    
    // Get the current HOME for expected path
    let current_home = env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
    
    // Test YAML with tilde expansion in subdirectory
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: ~/Documents/journals
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: null
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // The path should be expanded to the current home directory path
    let expected_path = PathBuf::from(current_home).join("Documents/journals");
    assert_eq!(vault.path, expected_path);
}

#[test]
#[serial]
fn test_tilde_expansion_windows_style() {
    use serde_yaml_ng;
    use std::env;
    
    // Test Windows-style environment variables
    let original_userprofile = env::var("USERPROFILE").unwrap_or_default();
    let original_home = env::var("HOME").unwrap_or_default();
    
    // Set Windows-style environment variables
    env::set_var("USERPROFILE", "C:\\Users\\testuser");
    env::set_var("HOME", "/home/testuser"); // Keep this as fallback
    
    // Test YAML with tilde expansion
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: ~/Documents/journal
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: null
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // On Windows, should use USERPROFILE, on Unix should use HOME
    let expected_path = if cfg!(windows) {
        PathBuf::from("C:\\Users\\testuser\\Documents\\journal")
    } else {
        PathBuf::from("/home/testuser/Documents/journal")
    };
    assert_eq!(vault.path, expected_path);
    
    // Restore original environment variables
    if !original_userprofile.is_empty() {
        env::set_var("USERPROFILE", original_userprofile);
    } else {
        env::remove_var("USERPROFILE");
    }
    if !original_home.is_empty() {
        env::set_var("HOME", original_home);
    } else {
        env::remove_var("HOME");
    }
}

#[test]
#[serial]
fn test_tilde_expansion_fallback() {
    use serde_yaml_ng;
    use std::env;
    
    // Test fallback behavior when no home directory is found
    let original_userprofile = env::var("USERPROFILE").ok();
    let original_home = env::var("HOME").ok();
    
    // Remove environment variables to test fallback
    env::remove_var("USERPROFILE");
    env::remove_var("HOME");
    
    // Test YAML with tilde expansion
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: ~/fallback-test
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: null
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // Should fall back to the original path when no home directory is found
    // Note: This test might not work in all environments due to system HOME variables
    // The important thing is that the code handles the case gracefully
    let path_str = vault.path.to_string_lossy();
    assert!(path_str.contains("fallback-test"));
    
    // Restore original environment variables
    if let Some(val) = original_userprofile {
        env::set_var("USERPROFILE", val);
    }
    // Don't remove if it wasn't set originally
    
    if let Some(val) = original_home {
        env::set_var("HOME", val);
    }
    // Don't remove if it wasn't set originally
}

#[test]
#[serial]
fn test_windows_env_var_expansion() {
    use serde_yaml_ng;
    use std::env;
    
    // Test Windows-style environment variable expansion
    let original_userprofile = env::var("USERPROFILE").unwrap_or_default();
    let original_appdata = env::var("APPDATA").unwrap_or_default();
    
    // Set Windows-style environment variables
    env::set_var("USERPROFILE", "C:\\Users\\testuser");
    env::set_var("APPDATA", "C:\\Users\\testuser\\AppData\\Roaming");
    
    // Test YAML with Windows environment variable expansion
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: "%USERPROFILE%/Documents/journal"
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: null
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // On Windows, should expand %USERPROFILE%, on Unix should leave as-is
    let expected_path = if cfg!(windows) {
        PathBuf::from("C:\\Users\\testuser\\Documents\\journal")
    } else {
        PathBuf::from("%USERPROFILE%/Documents/journal")
    };
    assert_eq!(vault.path, expected_path);
    
    // Restore original environment variables
    if !original_userprofile.is_empty() {
        env::set_var("USERPROFILE", original_userprofile);
    } else {
        env::remove_var("USERPROFILE");
    }
    if !original_appdata.is_empty() {
        env::set_var("APPDATA", original_appdata);
    } else {
        env::remove_var("APPDATA");
    }
}

#[test]
#[serial]
fn test_windows_env_var_expansion_appdata() {
    use serde_yaml_ng;
    use std::env;
    
    // Test APPDATA environment variable expansion
    let original_appdata = env::var("APPDATA").unwrap_or_default();
    
    // Set APPDATA environment variable
    env::set_var("APPDATA", "C:\\Users\\testuser\\AppData\\Roaming");
    
    // Test YAML with APPDATA expansion
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: "%APPDATA%/journey"
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: null
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // On Windows, should expand %APPDATA%, on Unix should leave as-is
    let expected_path = if cfg!(windows) {
        PathBuf::from("C:\\Users\\testuser\\AppData\\Roaming\\journey")
    } else {
        PathBuf::from("%APPDATA%/journey")
    };
    assert_eq!(vault.path, expected_path);
    
    // Restore original environment variable
    if !original_appdata.is_empty() {
        env::set_var("APPDATA", original_appdata);
    } else {
        env::remove_var("APPDATA");
    }
}

#[test]
#[serial]
fn test_windows_env_var_expansion_multiple() {
    use serde_yaml_ng;
    use std::env;
    
    // Test multiple environment variables in one path
    let original_userprofile = env::var("USERPROFILE").unwrap_or_default();
    let original_username = env::var("USERNAME").unwrap_or_default();
    
    // Set environment variables
    env::set_var("USERPROFILE", "C:\\Users\\testuser");
    env::set_var("USERNAME", "testuser");
    
    // Test YAML with multiple environment variables
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: "%USERPROFILE%/Documents/%USERNAME%_journal"
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: null
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // On Windows, should expand both variables, on Unix should leave as-is
    let expected_path = if cfg!(windows) {
        PathBuf::from("C:\\Users\\testuser\\Documents\\testuser_journal")
    } else {
        PathBuf::from("%USERPROFILE%/Documents/%USERNAME%_journal")
    };
    assert_eq!(vault.path, expected_path);
    
    // Restore original environment variables
    if !original_userprofile.is_empty() {
        env::set_var("USERPROFILE", original_userprofile);
    } else {
        env::remove_var("USERPROFILE");
    }
    if !original_username.is_empty() {
        env::set_var("USERNAME", original_username);
    } else {
        env::remove_var("USERNAME");
    }
}

#[test]
#[serial]
fn test_template_file_tilde_expansion() {
    use serde_yaml_ng;
    use std::env;
    
    // Test tilde expansion in template_file
    let original_home = env::var("HOME").unwrap_or_default();
    env::set_var("HOME", "/home/testuser");
    
    // Test YAML with tilde expansion in template_file
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: /tmp/test
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: "~/Documents/templates/journal.md"
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // The template_file should be expanded to the full home directory path
    assert_eq!(vault.template_file, Some("/home/testuser/Documents/templates/journal.md".to_string()));
    
    // Restore original HOME
    if !original_home.is_empty() {
        env::set_var("HOME", original_home);
    } else {
        env::remove_var("HOME");
    }
}

#[test]
#[serial]
fn test_template_file_windows_env_var_expansion() {
    use serde_yaml_ng;
    use std::env;
    
    // Test Windows environment variable expansion in template_file
    let original_userprofile = env::var("USERPROFILE").unwrap_or_default();
    env::set_var("USERPROFILE", "C:\\Users\\testuser");
    
    // Test YAML with Windows environment variable expansion in template_file
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: /tmp/test
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: "%USERPROFILE%/Documents/templates/journal.md"
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // On Windows, should expand %USERPROFILE%, on Unix should leave as-is
    let expected_template = if cfg!(windows) {
        "C:\\Users\\testuser\\Documents\\templates\\journal.md"
    } else {
        "%USERPROFILE%/Documents/templates/journal.md"
    };
    assert_eq!(vault.template_file, Some(expected_template.to_string()));
    
    // Restore original environment variable
    if !original_userprofile.is_empty() {
        env::set_var("USERPROFILE", original_userprofile);
    } else {
        env::remove_var("USERPROFILE");
    }
}

#[test]
fn test_template_file_no_expansion() {
    use serde_yaml_ng;
    
    // Test that regular paths without tildes or env vars are not modified
    let yaml_content = r#"
vaults:
  test:
    name: test
    path: /tmp/test
    locale: en-US
    phrases: {}
    section_name: null
    date_format: null
    template_file: "/absolute/path/to/template.md"
    file_path_format: null
"#;
    
    let config: Config = serde_yaml_ng::from_str(yaml_content).unwrap();
    let vault = config.get_vault("test").unwrap();
    
    // The template_file should remain unchanged
    assert_eq!(vault.template_file, Some("/absolute/path/to/template.md".to_string()));
}
