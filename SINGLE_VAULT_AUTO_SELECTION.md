# Single Vault Auto-Selection Feature

## Overview

The Journey CLI application now automatically selects the single vault when only one vault is configured, making the `--vault` flag optional in this scenario. This improves user experience by reducing the need to specify vault names when working with a single vault setup.

## Changes Made

### 1. Enhanced Vault Selection Logic

**File**: `src/app.rs` - `get_vault()` method

The vault selection logic now implements smart auto-selection:

```rust
fn get_vault(&self, vault_name: Option<&str>) -> Result<Vault, JourneyError> {
    let vault_config = if let Some(name) = vault_name {
        // User explicitly specified a vault
        self.config.get_vault(name)
            .ok_or_else(|| JourneyError::VaultNotFound(name.to_string()))?
    } else {
        // No vault specified - use smart selection
        if self.config.vaults.len() == 1 {
            // Only one vault exists - use it automatically
            self.config.vaults.values().next()
                .ok_or_else(|| JourneyError::VaultNotFound("No vaults configured".to_string()))?
        } else if self.config.vaults.len() > 1 {
            // Multiple vaults exist - need to specify which one
            return Err(JourneyError::VaultNotFound(
                format!("Multiple vaults available: {}. Please specify --vault", 
                    self.config.vaults.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
            ));
        } else {
            // No vaults configured
            return Err(JourneyError::VaultNotFound("No vaults configured. Use 'journey init' to create one.".to_string()));
        }
    };

    Ok(Vault::new(vault_config.clone()))
}
```

### 2. Updated CLI Help Text

**File**: `src/cli.rs`

The help text now indicates that the `--vault` flag is optional when only one vault exists:

```rust
/// Vault name to use (optional if only one vault exists)
#[arg(short, long)]
pub vault: Option<String>,
```

### 3. Enhanced ConfigManager

**File**: `src/config_manager.rs`

Added support for custom config paths via environment variable for testing:

```rust
pub fn new() -> Result<Self, JourneyError> {
    // Check if a custom config path is specified via environment variable
    let config_path = if let Ok(custom_path) = env::var("JOURNEY_CONFIG") {
        PathBuf::from(custom_path)
    } else {
        // Default config path logic...
    };
    
    Ok(Self { config_path })
}
```

### 4. Comprehensive Test Coverage

**File**: `tests/app_tests.rs`

Added 4 comprehensive tests covering all scenarios:

#### Test Cases:

1. **`test_single_vault_auto_selection()`**
   - Verifies that when only one vault exists, it's automatically selected
   - Tests both implicit (no `--vault`) and explicit vault selection

2. **`test_multiple_vaults_require_specification()`**
   - Verifies that when multiple vaults exist, the user must specify which one
   - Tests that explicit vault selection still works

3. **`test_no_vaults_configured()`**
   - Verifies that when no vaults are configured, appropriate error is shown
   - Tests error handling for non-existent vault names

4. **`test_app_creation()`**
   - Basic app creation test

## Behavior Scenarios

### Scenario 1: Single Vault (Auto-Selection)
```bash
# These commands work without --vault when only one vault exists
journey "My note"
journey --list
journey --edit
journey add "My note"
```

### Scenario 2: Multiple Vaults (Requires Specification)
```bash
# These commands require --vault when multiple vaults exist
journey --vault my-vault "My note"
journey --vault my-vault --list
journey --vault my-vault --edit
```

### Scenario 3: No Vaults Configured
```bash
# These commands show helpful error messages
journey "My note"
# Error: No vaults configured. Use 'journey init' to create one.
```

## Error Messages

The application now provides clear, helpful error messages:

1. **Multiple Vaults**: `"Multiple vaults available: vault1, vault2. Please specify --vault"`
2. **No Vaults**: `"No vaults configured. Use 'journey init' to create one."`
3. **Invalid Vault**: `"Vault 'invalid-name' not found"`

## Test Results

```
running 4 tests
test test_app_creation ... ok
test test_no_vaults_configured ... ok
test test_multiple_vaults_require_specification ... ok
test test_single_vault_auto_selection ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Benefits

1. **Improved UX**: Users with single vaults don't need to specify `--vault`
2. **Clear Errors**: Helpful error messages guide users to the correct solution
3. **Backward Compatibility**: Existing workflows continue to work
4. **Flexible**: Supports both single and multiple vault scenarios
5. **Well Tested**: Comprehensive test coverage ensures reliability

## Usage Examples

### Single Vault Setup
```bash
# Initialize a vault
journey init --path ~/my-journal --name personal

# Use without --vault (auto-selected)
journey "Had a great day!"
journey --list
journey --edit
```

### Multiple Vault Setup
```bash
# Initialize multiple vaults
journey init --path ~/work-journal --name work
journey init --path ~/personal-journal --name personal

# Must specify vault
journey --vault work "Meeting notes"
journey --vault personal "Weekend plans"
```

The single vault auto-selection feature significantly improves the user experience for the common case of having one vault while maintaining full functionality for multiple vault scenarios.
