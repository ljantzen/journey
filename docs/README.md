# Journey Documentation

This directory contains detailed documentation for the Journey CLI application.

## User Guides

### Getting Started
- **[../README.md](../README.md)** - Main application documentation, quick start, and basic usage

### Core Features

#### Vault Management
- **[SINGLE_VAULT_AUTO_SELECTION.md](SINGLE_VAULT_AUTO_SELECTION.md)** - Single vault auto-selection feature
- **[OPTIONAL_NAME_INIT.md](OPTIONAL_NAME_INIT.md)** - Optional `--name` parameter for init command
- **[CONFIG_FILE_VALIDATION.md](CONFIG_FILE_VALIDATION.md)** - Config file existence validation and init prompt

#### Note Organization
- **[CATEGORIES.md](CATEGORIES.md)** - Category-based section organization (work, personal, health, meetings)
- **[SECTION_BASED_NOTES.md](SECTION_BASED_NOTES.md)** - Section-based note organization (legacy documentation)
- **[TABLE_MODE.md](TABLE_MODE.md)** - Table format, headers, and locale-aware display
- **[PHRASE_EXPANSION.md](PHRASE_EXPANSION.md)** - Phrase shortcuts and expansion

#### Customization
- **[TEMPLATE_VARIABLES.md](TEMPLATE_VARIABLES.md)** - Template file variables and substitution
- **[FILE_PATH_FORMATS.md](FILE_PATH_FORMATS.md)** - Custom file path formats and organization
- **[LOCALIZATION.md](LOCALIZATION.md)** - Locale support and internationalization

#### Date & Time
- **[DATE_FORMAT_OVERRIDE.md](DATE_FORMAT_OVERRIDE.md)** - Custom date format configuration
- **[TIME_FORMAT_OVERRIDE.md](TIME_FORMAT_OVERRIDE.md)** - Time format override (12h/24h)
- **[TIMESTAMP_FORMAT_CHANGE.md](TIMESTAMP_FORMAT_CHANGE.md)** - Timestamp format changes
- **[LOCALE_TESTING.md](LOCALE_TESTING.md)** - Locale-aware date/time parsing and testing

#### Integration
- **[OBSIDIAN_INTEGRATION.md](OBSIDIAN_INTEGRATION.md)** - Obsidian vault integration and plugin support
- **[ENVIRONMENT_VARIABLE_MERGE.md](ENVIRONMENT_VARIABLE_MERGE.md)** - Environment variable consolidation

#### Behavior
- **[DEFAULT_BEHAVIOR_UPDATE.md](DEFAULT_BEHAVIOR_UPDATE.md)** - Default behavior changes

### Testing & Development
- **[TEST_SUMMARY.md](TEST_SUMMARY.md)** - Test coverage and results
- **[TEST_REORGANIZATION.md](TEST_REORGANIZATION.md)** - Moving tests to separate files
- **[TEST_ISOLATION_SAFETY.md](TEST_ISOLATION_SAFETY.md)** - Test isolation safety mechanisms

## Quick Reference

### Key Features

| Feature | Documentation | Description |
|---------|---------------|-------------|
| Categories | [CATEGORIES.md](CATEGORIES.md) | Organize notes by category (work, personal, health, meetings) |
| Table Mode | [TABLE_MODE.md](TABLE_MODE.md) | Display notes in table format with headers |
| Templates | [TEMPLATE_VARIABLES.md](TEMPLATE_VARIABLES.md) | Custom templates with variable substitution |
| Phrases | [PHRASE_EXPANSION.md](PHRASE_EXPANSION.md) | Define shortcuts that expand to full text |
| File Paths | [FILE_PATH_FORMATS.md](FILE_PATH_FORMATS.md) | Customize note file organization |
| Obsidian | [OBSIDIAN_INTEGRATION.md](OBSIDIAN_INTEGRATION.md) | Import from Obsidian vaults |
| Localization | [LOCALIZATION.md](LOCALIZATION.md) | Multi-language support |

### Configuration Examples

#### Basic Vault
```yaml
vaults:
  personal:
    name: personal
    path: ~/Documents/journal
    locale: en_US.UTF-8
    list_type: bullet
    section_header: "Daily Log"
```

#### Advanced Vault with Categories
```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    locale: en_US.UTF-8
    
    # File organization
    file_path_format: "{year}/{month:02}/{date:02}.md"
    template_file: ~/templates/work-daily.md
    
    # Note format
    list_type: table
    table_headers:
      time: "Time"
      content: "Content"
    
    # Category sections
    section_header: "General Notes"
    section_header_work: "Work Tasks"
    section_header_personal: "Personal Notes"
    section_header_health: "Health & Fitness"
    section_header_meetings: "Meeting Notes"
    
    # Phrase expansion
    phrases:
      "@meeting": "Team meeting"
      "@standup": "Daily standup"
      "@review": "Code review"
```

#### Obsidian-Compatible Vault
```yaml
vaults:
  obsidian:
    name: obsidian
    path: ~/Documents/ObsidianVault
    locale: en_US.UTF-8
    date_format: YYYY-MM-DD
    file_path_format: "Daily Notes/{year}-{month:02}-{date:02}.md"
    template_file: "Templates/Daily Note Template.md"
```

### Usage Examples

#### Basic Usage
```bash
# Add a note
journey My note content

# List today's notes
journey

# Edit today's notes
journey --edit
```

#### With Categories
```bash
# Add categorized notes
journey -c work "Completed deployment"
journey -c personal "Dinner with family"
journey -c health "30 minutes of exercise"

# List categorized notes
journey --list -c work
```

#### With Dates and Times
```bash
# Add note for yesterday
journey --relative-date 1 "Forgot to log this"

# Add note with specific time
journey --time 14:30 "Afternoon meeting"

# List notes for specific date
journey --date 2025-10-24 --list
```

#### Table Mode
```bash
# List with table headers
journey --list --header

# Export to file
journey --list --header > daily-report.md
```

#### Phrase Expansion
```bash
# Use phrase shortcuts
journey "@meeting went well"
# Expands to: "Team meeting went well"
```

## Feature Documentation Structure

Each feature documentation file follows this structure:

1. **Overview** - What the feature does
2. **Configuration** - How to configure it
3. **Usage** - How to use it with examples
4. **Use Cases** - Common scenarios
5. **Best Practices** - Recommendations
6. **Troubleshooting** - Common issues and solutions

## Contributing to Documentation

When adding new features:

1. Create a new `.md` file in `docs/` with the feature name
2. Follow the standard structure above
3. Add the file to this index
4. Update the main README.md with a brief mention and link
5. Include practical examples and use cases

## Documentation Standards

- Use clear, descriptive headings
- Include code examples with syntax highlighting
- Provide both simple and advanced examples
- Document error cases and troubleshooting
- Keep examples up-to-date with the codebase
- Use tables for reference information
- Include cross-references to related documentation

## Getting Help

- Check the main [README.md](../README.md) for quick start and basic usage
- Browse feature-specific documentation in this directory
- Review examples in each documentation file
- Check the [TEST_SUMMARY.md](TEST_SUMMARY.md) for test coverage details

## Version History

Documentation is maintained alongside the codebase to ensure accuracy. Each feature document includes implementation details and version information where relevant.
