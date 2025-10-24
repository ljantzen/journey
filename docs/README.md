# Journey Documentation

This directory contains detailed documentation for the Journey CLI application.

## Core Features

### Configuration & Setup
- **[CONFIG_FILE_VALIDATION.md](CONFIG_FILE_VALIDATION.md)** - Config file existence validation and init prompt
- **[SINGLE_VAULT_AUTO_SELECTION.md](SINGLE_VAULT_AUTO_SELECTION.md)** - Single vault auto-selection feature
- **[OPTIONAL_NAME_INIT.md](OPTIONAL_NAME_INIT.md)** - Optional `--name` parameter for init command

### Date & Time Handling
- **[DATE_FORMAT_OVERRIDE.md](DATE_FORMAT_OVERRIDE.md)** - Date format override in journey.yaml
- **[TIME_FORMAT_OVERRIDE.md](TIME_FORMAT_OVERRIDE.md)** - Time format override functionality
- **[TIMESTAMP_FORMAT_CHANGE.md](TIMESTAMP_FORMAT_CHANGE.md)** - Timestamp format changes
- **[LOCALE_TESTING.md](LOCALE_TESTING.md)** - Locale-aware date/time parsing and testing

### Behavior & Features
- **[DEFAULT_BEHAVIOR_UPDATE.md](DEFAULT_BEHAVIOR_UPDATE.md)** - Default behavior changes
- **[ENVIRONMENT_VARIABLE_MERGE.md](ENVIRONMENT_VARIABLE_MERGE.md)** - Environment variable consolidation
- **[SECTION_BASED_NOTES.md](SECTION_BASED_NOTES.md)** - Section-based note organization
- **[TEMPLATE_FILES.md](TEMPLATE_FILES.md)** - Custom template files for new markdown files

### Testing & Development
- **[TEST_SUMMARY.md](TEST_SUMMARY.md)** - Test coverage and results
- **[TEST_REORGANIZATION.md](TEST_REORGANIZATION.md)** - Moving tests to separate files
- **[TEST_ISOLATION_SAFETY.md](TEST_ISOLATION_SAFETY.md)** - Test isolation safety mechanisms

## Quick Reference

### Main Application
- **[../README.md](../README.md)** - Main application documentation and usage guide

### Key Features
- **Date Format Override**: Configure custom date formats in `journey.yaml`
- **Time Format Override**: Force 12h or 24h time format with `--time-format`
- **Stdin Support**: Read input from stdin with `--stdin`
- **Locale Support**: Automatic locale detection for date/time parsing
- **Single Vault Auto-Selection**: Automatic vault selection when only one exists
- **Section-Based Notes**: Organize notes into specific markdown sections
- **Template Files**: Custom templates for new markdown files with variable substitution

### Configuration Examples
```yaml
# journey.yaml
vaults:
  my-vault:
    name: my-vault
    path: /path/to/vault
    locale: en_US.UTF-8
    phrases: {}
    section_name: null
    date_format: YYYY-MM-DD  # Optional date format override
    template_file: /path/to/template.md  # Optional template file
```

### Usage Examples
```bash
# Basic usage
journey My note content

# With time override
journey --time 2:30PM --time-format 12h Meeting notes

# From stdin
echo -e "Note 1\nNote 2" | journey --stdin

# With date
journey --date 2025-10-24 Note for specific date
```

## Development

All documentation is maintained alongside the codebase to ensure accuracy and completeness. Each feature has its own documentation file explaining implementation details, usage examples, and testing approaches.
