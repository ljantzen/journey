# Configuration File Validation

## Overview

The Journey CLI application now validates the existence of the `journey.yaml` configuration file and provides helpful error messages when it's missing, while still allowing the `init` command to work for first-time setup.

## Changes Made

### 1. Enhanced App Initialization

**File**: `src/app.rs` - `App::new()` method

The application now checks for the existence of the configuration file before proceeding:

```rust
pub fn new() -> Result<Self, JourneyError> {
    let config_manager = ConfigManager::new()?;
    
    // Check if config file exists
    if !config_manager.config_exists() {
        return Err(JourneyError::Config(
            "No configuration file found. Please run 'journey init' to create your first vault.".to_string()
        ));
    }
    
    let config = config_manager.load_config()?;
    
    Ok(Self {
        config_manager,
        config,
    })
}
```

### 2. Special Initialization for Init Command

**File**: `src/app.rs` - `App::new_for_init()` method

Added a special initialization method that doesn't require the config file to exist:

```rust
pub fn new_for_init() -> Result<Self, JourneyError> {
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load_config()?;
    
    Ok(Self {
        config_manager,
        config,
    })
}
```

### 3. Smart Command Detection

**File**: `src/main.rs`

Modified the main function to detect init commands and use the appropriate initialization method:

```rust
fn main() {
    let cli = Cli::parse();
    
    // Check if this is an init command - if so, use special initialization
    let app_result = if matches!(cli.command, Some(journey::cli::Commands::Init { .. })) {
        App::new_for_init()
    } else {
        App::new()
    };
    
    match app_result {
        Ok(mut app) => {
            if let Err(e) = app.run(cli) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    }
}
```

### 4. Comprehensive Test Coverage

**File**: `tests/app_tests.rs`

Added test to verify the new behavior:

```rust
#[test]
fn test_no_config_file_exists() {
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
        assert!(msg.contains("journey init"));
    } else {
        panic!("Expected Config error");
    }
}
```

## Behavior Scenarios

### Scenario 1: No Configuration File (Most Commands)
```bash
# These commands fail with helpful error message
journey "My note"
journey --list
journey --edit
journey add "My note"

# Output:
# Failed to initialize application: Configuration error: No configuration file found. Please run 'journey init' to create your first vault.
```

### Scenario 2: No Configuration File (Init Command)
```bash
# This command works even without config file
journey init --path ~/my-journal --name personal

# Output:
# Vault 'personal' initialized successfully!
```

### Scenario 3: Configuration File Exists
```bash
# All commands work normally after initialization
journey "My note"
journey --list
journey --edit
journey add "My note"
```

## Error Messages

The application now provides clear, actionable error messages:

1. **Missing Config File**: `"No configuration file found. Please run 'journey init' to create your first vault."`
2. **Standard Errors**: All other error messages remain unchanged

## Test Results

```
running 5 tests
test test_no_config_file_exists ... ok
test test_app_creation ... ok
test test_no_vaults_configured ... ok
test test_multiple_vaults_require_specification ... ok
test test_single_vault_auto_selection ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Benefits

1. **Clear User Guidance**: Users immediately know what to do when no config exists
2. **Prevents Confusion**: No more mysterious errors when trying to use uninitialized application
3. **Maintains Functionality**: Init command still works for first-time setup
4. **Backward Compatibility**: Existing workflows continue to work unchanged
5. **Well Tested**: Comprehensive test coverage ensures reliability

## Usage Examples

### First-Time Setup
```bash
# User tries to use journey without setup
journey "My first note"
# Error: No configuration file found. Please run 'journey init' to create your first vault.

# User follows the guidance
journey init --path ~/my-journal --name personal
# Vault 'personal' initialized successfully!

# Now all commands work
journey "My first note"
# Note added successfully!
```

### Existing Setup
```bash
# All commands work normally
journey "My note"
journey --list
journey --edit
journey add "Another note"
```

The configuration file validation feature significantly improves the user experience by providing clear guidance when the application hasn't been initialized, while maintaining full functionality for the init command and existing setups.
