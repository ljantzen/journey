# Date Format Override Feature

## Overview

The date format override feature allows you to specify a custom date format in your `journey.yaml` config file. This ensures consistent date parsing regardless of your system locale, providing validation and standardization for date inputs.

## Configuration

Add a `date_format` field to your vault configuration in `journey.yaml`:

```yaml
vaults:
  journey:
    name: journey
    path: /tmp/test-journey
    locale: en_US.UTF-8
    phrases: {}
    section_name: null
    date_format: YYYY-MM-DD  # Custom date format override
```

## Supported Date Formats

### Predefined Format Names
- `YYYY-MM-DD` - ISO format: `2025-10-24`
- `MM/DD/YYYY` - US format: `10/24/2025`
- `DD.MM.YYYY` - European format: `24.10.2025`
- `DD/MM/YYYY` - European format: `24/10/2025`
- `MM-DD-YYYY` - US with dashes: `10-24-2025`
- `DD-MM-YYYY` - European with dashes: `24-10-2025`

### Custom Chrono Format Strings
You can also use any valid chrono format string:
- `%Y/%m/%d` - Custom: `2025/10/24`
- `%d-%m-%Y` - Custom: `24-10-2025`
- `%B %d, %Y` - Long format: `October 24, 2025`

## Usage Examples

### ISO Format Override
```yaml
# journey.yaml
vaults:
  journey:
    date_format: YYYY-MM-DD
```

```bash
# Only accepts ISO format dates
journey --date "2025-10-24" "Note with ISO date"
# This would fail: journey --date "10/24/2025" "Note"
```

### US Format Override
```yaml
# journey.yaml
vaults:
  journey:
    date_format: MM/DD/YYYY
```

```bash
# Only accepts US format dates
journey --date "10/24/2025" "Note with US date"
# This would fail: journey --date "24.10.2025" "Note"
```

### European Format Override
```yaml
# journey.yaml
vaults:
  journey:
    date_format: DD.MM.YYYY
```

```bash
# Only accepts European format dates
journey --date "24.10.2025" "Note with European date"
# This would fail: journey --date "10/24/2025" "Note"
```

## Benefits

1. **Consistency**: Force a specific date format across different locales
2. **Validation**: Ensure only certain date formats are accepted
3. **Integration**: Match external systems that use specific date formats
4. **Team Standardization**: Standardize date input across team members

## Error Handling

If you specify a date that doesn't match the configured format, Journey will return an error:

```bash
# With date_format: YYYY-MM-DD
journey --date "10/24/2025" "This will fail"
# Error: Could not parse date: 10/24/2025 with format override: YYYY-MM-DD
```

## Fallback Behavior

If no `date_format` is specified in the config (or it's `null`), Journey falls back to locale-based date parsing:

- **English locales**: Tries US formats first (`10/24/2025`), then ISO (`2025-10-24`)
- **Norwegian locales**: Tries European formats first (`24.10.2025`), then ISO (`2025-10-24`)
- **Other locales**: Tries a mix of common formats

## Implementation Details

### Config Structure
```rust
pub struct VaultConfig {
    pub name: String,
    pub path: PathBuf,
    pub locale: String,
    pub phrases: HashMap<String, String>,
    pub section_name: Option<String>,
    pub date_format: Option<String>,  // New field
}
```

### Date Parsing Logic
```rust
pub fn parse_date_with_format_override(&self, date_str: &str, format_override: Option<&str>) -> Result<NaiveDate, JourneyError> {
    if let Some(override_format) = format_override {
        // Use only the specified format
        let format_str = match override_format {
            "YYYY-MM-DD" => "%Y-%m-%d",
            "MM/DD/YYYY" => "%m/%d/%Y",
            // ... other predefined formats
            _ => override_format, // Use as-is for custom chrono formats
        };
        // Parse with single format
    } else {
        // Fall back to locale-based parsing
    }
}
```

## Use Cases

### 1. Team Standardization
```yaml
# All team members use ISO format
vaults:
  team-journal:
    date_format: YYYY-MM-DD
```

### 2. Integration with External Systems
```yaml
# Match external system's date format
vaults:
  integration-vault:
    date_format: MM/DD/YYYY
```

### 3. Validation for Data Entry
```yaml
# Ensure strict date format compliance
vaults:
  data-entry:
    date_format: DD.MM.YYYY
```

### 4. Custom Format Requirements
```yaml
# Use custom chrono format
vaults:
  custom-vault:
    date_format: "%Y/%m/%d"  # 2025/10/24
```

## Migration

To add date format override to existing vaults:

1. **Edit your `journey.yaml`**:
   ```yaml
   vaults:
     journey:
       name: journey
       path: /path/to/your/actual/vault
       locale: en_US.UTF-8
       phrases: {}
       section_name: null
       date_format: YYYY-MM-DD  # Add this line
   ```

2. **Test the new format**:
   ```bash
   journey --date "2025-10-24" "Test with new format"
   ```

3. **Verify it works as expected**:
   ```bash
   journey --list
   ```

## Backward Compatibility

- ✅ Existing vaults without `date_format` continue to work unchanged
- ✅ Locale-based date parsing still works as before
- ✅ All existing commands and options remain functional
- ✅ New `date_format` field is optional (`Option<String>`)

## Testing

The feature includes comprehensive tests covering:
- ✅ ISO format override
- ✅ US format override  
- ✅ European format override
- ✅ Custom chrono format strings
- ✅ Format mismatch error handling
- ✅ Fallback to locale-based parsing
- ✅ Cross-locale compatibility
