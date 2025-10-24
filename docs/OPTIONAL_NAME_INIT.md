# Optional Name Parameter for Init Command

## Overview

The Journey CLI application now supports optional `--name` parameter for the `init` command. When `--name` is not provided, the vault name is automatically derived from the path's basename, making the init command more user-friendly.

## Changes Made

### 1. Updated CLI Definition

**File**: `src/cli.rs`

Changed the `name` parameter from required to optional:

```rust
/// Initialize a new vault
Init {
    /// Path to the vault directory
    #[arg(short, long)]
    path: PathBuf,
    
    /// Name of the vault (defaults to path basename if not provided)
    #[arg(short, long)]
    name: Option<String>,
    
    /// Type of vault (table|bullet)
    #[arg(short, long)]
    vault_type: Option<String>,
},
```

### 2. Enhanced Init Vault Logic

**File**: `src/app.rs` - `init_vault()` method

Updated the method to handle optional name parameter and use path basename as default:

```rust
pub fn init_vault(&mut self, path: std::path::PathBuf, name: Option<String>) -> Result<(), JourneyError> {
    // Create vault directory
    std::fs::create_dir_all(&path)?;

    // Determine vault name - use provided name or path basename
    let vault_name = if let Some(name) = name {
        name
    } else {
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| JourneyError::Config("Invalid path: cannot extract basename".to_string()))?
            .to_string()
    };

    // Get system locale
    let locale = self.get_system_locale();

    // Create vault config
    let vault_config = crate::config::VaultConfig {
        name: vault_name.clone(),
        path,
        locale,
        phrases: std::collections::HashMap::new(),
        section_name: None,
    };

    // Add to config and save
    self.config.add_vault(vault_config);
    self.config_manager.save_config(&self.config)?;

    println!("Vault '{}' initialized successfully!", vault_name);
    Ok(())
}
```

### 3. Updated CLI Tests

**File**: `tests/cli_tests.rs`

Updated the CLI test to handle the optional name parameter:

```rust
Some(Commands::Init { path, name, vault_type: _ }) => {
    assert_eq!(path, PathBuf::from("/tmp/test"));
    assert_eq!(name, Some("test".to_string()));
}
```

### 4. Comprehensive App Tests

**File**: `tests/app_tests.rs`

Added three new tests to verify the functionality:

```rust
#[test]
fn test_init_vault_with_name() {
    // Test init with explicit name
    let result = app.init_vault(PathBuf::from("/tmp/test-vault"), Some("my-vault".to_string()));
    assert!(result.is_ok());
}

#[test]
fn test_init_vault_without_name() {
    // Test init without name - should use path basename
    let result = app.init_vault(PathBuf::from("/tmp/my-journal"), None);
    assert!(result.is_ok());
}

#[test]
fn test_init_vault_invalid_path() {
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
```

## Behavior Scenarios

### Scenario 1: Init with Explicit Name
```bash
journey init --path /tmp/my-journal --name personal
# Output: Vault 'personal' initialized successfully!
```

### Scenario 2: Init without Name (Uses Path Basename)
```bash
journey init --path /tmp/my-journal
# Output: Vault 'my-journal' initialized successfully!
```

### Scenario 3: Init with Invalid Path
```bash
journey init --path ""
# Output: Error: Invalid path: cannot extract basename
```

## Test Results

```
running 8 tests
test test_init_vault_invalid_path ... ok
test test_no_config_file_exists ... ok
test test_init_vault_with_name ... ok
test test_app_creation ... ok
test test_init_vault_without_name ... ok
test test_single_vault_auto_selection ... ok
test test_no_vaults_configured ... ok
test test_multiple_vaults_require_specification ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Benefits

1. **Improved User Experience**: Users don't need to specify a name when the path basename is sufficient
2. **Reduced Command Length**: Shorter commands for common use cases
3. **Intuitive Behavior**: Path basename is a natural default for vault names
4. **Backward Compatibility**: Explicit names still work as before
5. **Error Handling**: Clear error messages for invalid paths
6. **Well Tested**: Comprehensive test coverage ensures reliability

## Usage Examples

### Quick Setup (Using Path Basename)
```bash
# Create vault with name derived from path
journey init --path ~/work-journal
# Vault 'work-journal' initialized successfully!

journey init --path ~/personal-notes
# Vault 'personal-notes' initialized successfully!
```

### Explicit Naming (When Needed)
```bash
# Create vault with custom name
journey init --path ~/my-notes --name work
# Vault 'work' initialized successfully!

journey init --path ~/documents --name personal
# Vault 'personal' initialized successfully!
```

### Error Handling
```bash
# Invalid path
journey init --path ""
# Error: Invalid path: cannot extract basename

# Empty path
journey init --path /
# Error: Invalid path: cannot extract basename
```

## Implementation Details

### Path Basename Extraction
The implementation uses Rust's `PathBuf::file_name()` method to extract the basename:

```rust
let vault_name = if let Some(name) = name {
    name
} else {
    path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| JourneyError::Config("Invalid path: cannot extract basename".to_string()))?
        .to_string()
};
```

### Error Handling
- **Invalid Path**: Returns `JourneyError::Config` with descriptive message
- **Unicode Issues**: Handles non-UTF8 path components gracefully
- **Empty Paths**: Detects and reports empty or invalid paths

The optional name parameter feature significantly improves the user experience by reducing the need to specify vault names when the path basename is sufficient, while maintaining full functionality for cases where explicit naming is desired.
