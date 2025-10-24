# Time Format Override Feature

## Overview

The time format override feature allows users to force a specific time format (12-hour or 24-hour) regardless of their locale settings. This provides consistency and validation when working with time inputs.

## Implementation

### CLI Argument
- **Option**: `--time-format <FORMAT>`
- **Values**: `12h` or `24h`
- **Usage**: `journey --time "2:30 PM" --time-format "12h" "Note"`

### Core Changes

#### 1. CLI Interface (`src/cli.rs`)
```rust
/// Time format override (12h|24h)
#[arg(long)]
pub time_format: Option<String>,
```

#### 2. DateTimeHandler (`src/date_time.rs`)
- Added `parse_time_with_format_override()` method
- Supports format-specific parsing:
  - `12h`: Only accepts `2:30 PM`, `2:30:45 PM`, `2:30PM`, `2:30:45PM`
  - `24h`: Only accepts `14:30`, `14:30:45`
- Maintains backward compatibility with existing `parse_time()` method

#### 3. App Logic (`src/app.rs`)
- Updated `CliArgs` struct to include `time_format`
- Modified `parse_time()` to use format override when specified
- Passes format override through all command handlers

## Usage Examples

### 12-Hour Format Override
```bash
# Force 12-hour format parsing
journey --time "2:30 PM" --time-format "12h" "Meeting at 2:30 PM"
journey --time "2:30:45 PM" --time-format "12h" "Precise time"
journey --time "2:30PM" --time-format "12h" "Compact format"
```

### 24-Hour Format Override
```bash
# Force 24-hour format parsing
journey --time "14:30" --time-format "24h" "Meeting at 14:30"
journey --time "14:30:45" --time-format "24h" "Precise time"
```

### Error Handling
```bash
# These will fail with format mismatch
journey --time "2:30 PM" --time-format "24h" "This will fail"
journey --time "14:30" --time-format "12h" "This will fail"
```

## Use Cases

1. **Consistency**: Force a specific format across different locales
2. **Validation**: Ensure only certain time formats are accepted
3. **Integration**: Match external systems that use specific time formats
4. **Workflow**: Standardize time input in team environments

## Testing

### Test Coverage
- ✅ 12-hour format override with valid times
- ✅ 24-hour format override with valid times
- ✅ Invalid format override values
- ✅ Format mismatch error handling
- ✅ Fallback to locale-based parsing when no override

### Test Files
- `tests/date_time_tests.rs`: Core format override functionality
- All existing tests continue to pass

## Backward Compatibility

- ✅ Existing `--time` usage without `--time-format` works unchanged
- ✅ Locale-based format detection still works as before
- ✅ All existing commands and options remain functional

## Benefits

1. **Flexibility**: Users can choose their preferred time format
2. **Validation**: Prevents ambiguous time inputs
3. **Consistency**: Standardizes time format across different environments
4. **Error Prevention**: Clear error messages for format mismatches

## Technical Details

### Format Override Logic
```rust
pub fn parse_time_with_format_override(&self, time_str: &str, format_override: Option<&str>) -> Result<NaiveTime, JourneyError> {
    if let Some(override_format) = format_override {
        let formats = match override_format {
            "12h" => vec![/* 12-hour formats */],
            "24h" => vec![/* 24-hour formats */],
            _ => return Err(/* Invalid format error */)
        };
        // Try only the specified format
    } else {
        // Fall back to locale-based parsing
    }
}
```

### Error Messages
- Invalid format: `"Invalid time format override: {}. Use '12h' or '24h'"`
- Format mismatch: `"Could not parse time: {} with format override: {}"`

## Future Enhancements

Potential future improvements:
- Support for additional time formats (e.g., `ISO`, `RFC3339`)
- Timezone-aware format overrides
- Custom format string support
- Batch time format validation
