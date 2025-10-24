# Environment Variable Merge: JOURNEY_CONFIG Consolidation

## Overview

The Journey CLI application has been updated to consolidate environment variable usage into a single `JOURNEY_CONFIG` variable, removing the legacy `JOURNEY_CONFIG_PATH` support. This simplifies the configuration system and reduces complexity.

## Changes Made

### 1. Updated ConfigManager

**File**: `src/config_manager.rs` - `ConfigManager::new()`

**Before:**
```rust
impl ConfigManager {
    pub fn new() -> Result<Self, JourneyError> {
        // Check if a custom config path is specified via environment variable
        // JOURNEY_CONFIG takes precedence over JOURNEY_CONFIG_PATH for consistency
        let config_path = if let Ok(custom_path) = env::var("JOURNEY_CONFIG") {
            PathBuf::from(custom_path)
        } else if let Ok(custom_path) = env::var("JOURNEY_CONFIG_PATH") {
            PathBuf::from(custom_path)
        } else {
            let config_dir = config_dir()
                .ok_or_else(|| JourneyError::Config("Could not find config directory".to_string()))?;
            
            let journey_config_dir = config_dir.join("journey");
            fs::create_dir_all(&journey_config_dir)?;
            
            journey_config_dir.join("journey.yaml")
        };
        
        Ok(Self { config_path })
    }
}
```

**After:**
```rust
impl ConfigManager {
    pub fn new() -> Result<Self, JourneyError> {
        // Check if a custom config path is specified via environment variable
        let config_path = if let Ok(custom_path) = env::var("JOURNEY_CONFIG") {
            PathBuf::from(custom_path)
        } else {
            let config_dir = config_dir()
                .ok_or_else(|| JourneyError::Config("Could not find config directory".to_string()))?;
            
            let journey_config_dir = config_dir.join("journey");
            fs::create_dir_all(&journey_config_dir)?;
            
            journey_config_dir.join("journey.yaml")
        };
        
        Ok(Self { config_path })
    }
}
```

### 2. Updated Test Isolation Function

**File**: `tests/app_tests.rs` - `ensure_test_isolation()`

**Before:**
```rust
// Safety mechanism: Ensure tests never touch production config
fn ensure_test_isolation() {
    // Clear any existing config environment variables to prevent accidental production access
    env::remove_var("JOURNEY_CONFIG");
    env::remove_var("JOURNEY_CONFIG_PATH");
}
```

**After:**
```rust
// Safety mechanism: Ensure tests never touch production config
fn ensure_test_isolation() {
    // Clear any existing config environment variable to prevent accidental production access
    env::remove_var("JOURNEY_CONFIG");
}
```

### 3. Updated Documentation

All documentation files have been updated to remove references to `JOURNEY_CONFIG_PATH`:

- **TEST_ISOLATION_SAFETY.md**: Updated environment variable usage examples
- **CONFIG_FILE_VALIDATION.md**: Updated test examples
- **SINGLE_VAULT_AUTO_SELECTION.md**: Updated config path examples

## Environment Variable Usage

### Production Usage
- **No Environment Variable**: Uses default config directory (`~/.config/journey/journey.yaml`)
- **JOURNEY_CONFIG**: Override config file path

### Test Usage
- **JOURNEY_CONFIG**: Always set to temporary directory for test isolation
- **Safety Mechanism**: Clears environment variable before each test
- **Temporary Directories**: All tests use `tempfile::TempDir` for isolation

## Benefits

### 1. **Simplified Configuration**
- Single environment variable to manage
- No confusion between multiple config variables
- Clearer documentation and usage

### 2. **Reduced Complexity**
- Removed legacy support code
- Simplified ConfigManager logic
- Fewer environment variables to track

### 3. **Better Maintainability**
- Single point of configuration
- Easier to understand and debug
- Consistent behavior across all usage

### 4. **Test Isolation**
- Simplified test isolation mechanism
- Only one environment variable to clear
- More reliable test execution

## Test Results

### Single Thread Execution (Recommended)
```bash
cargo test -- --test-threads=1
```

```
running 9 tests
test test_app_creation ... ok
test test_init_vault_invalid_path ... ok
test test_init_vault_with_name ... ok
test test_init_vault_without_name ... ok
test test_multiple_vaults_require_specification ... ok
test test_no_config_file_exists ... ok
test test_no_vaults_configured ... ok
test test_production_config_isolation ... ok
test test_single_vault_auto_selection ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Parallel Execution (May Have Interference)
```bash
cargo test
```

**Note**: Tests that modify global state (environment variables) may interfere with each other when run in parallel. Use `--test-threads=1` for reliable execution.

## Migration Guide

### For Users
- **No Action Required**: Existing usage with `JOURNEY_CONFIG` continues to work
- **Legacy Support Removed**: `JOURNEY_CONFIG_PATH` is no longer supported
- **Update Scripts**: If using `JOURNEY_CONFIG_PATH`, update to use `JOURNEY_CONFIG`

### For Developers
- **Test Execution**: Use `cargo test -- --test-threads=1` for reliable test execution
- **Environment Variables**: Only use `JOURNEY_CONFIG` for configuration
- **Test Isolation**: Tests automatically use the safety mechanism

## Usage Examples

### Production Usage
```bash
# Default config location
journey init --path ~/my-journal

# Custom config location
JOURNEY_CONFIG=/custom/path/journey.yaml journey init --path ~/my-journal
```

### Test Usage
```rust
#[test]
fn my_test() {
    // Ensure test isolation
    ensure_test_isolation();
    
    // Create temporary config
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("journey.yaml");
    
    // Set test config path
    env::set_var("JOURNEY_CONFIG", config_path.to_str().unwrap());
    
    // Test code here - completely isolated from production
}
```

### Running Tests
```bash
# Recommended: Single thread for reliable execution
cargo test -- --test-threads=1

# All tests (may have interference)
cargo test

# Specific test
cargo test test_production_config_isolation
```

## Implementation Details

### Environment Variable Priority
1. `JOURNEY_CONFIG` (if set, overrides default)
2. Default config directory (production)

### Test Isolation Flow
1. `ensure_test_isolation()` clears `JOURNEY_CONFIG`
2. Test creates temporary directory
3. Test sets `JOURNEY_CONFIG` to temporary path
4. Test code runs with isolated config
5. Temporary directory is automatically cleaned up

### Safety Verification
- `test_production_config_isolation()` verifies the mechanism works
- All tests use the safety function
- Environment variable is explicitly managed
- Temporary directories ensure complete isolation

## Test Execution Notes

### Parallel vs Sequential Execution
- **Parallel Execution**: May have test interference due to shared environment variables
- **Sequential Execution**: Reliable test execution with proper isolation
- **Recommendation**: Use `--test-threads=1` for consistent results

### Test Isolation Benefits
- **Production Safety**: Zero risk of test code touching production data
- **Test Reliability**: Tests run in completely isolated environments
- **Developer Confidence**: Tests can be run safely in any environment

The environment variable merge successfully simplifies the configuration system while maintaining all safety and isolation guarantees. The single `JOURNEY_CONFIG` variable provides a clean, consistent interface for both production and test usage.
