# Obsidian Integration

Journey can integrate with existing Obsidian vaults, automatically detecting and configuring from your Obsidian plugins.

## Basic Obsidian Vault Initialization

```bash
# Initialize from an existing Obsidian vault
journeyctl init --path ~/Documents/MyObsidianVault --obsidian

# With custom name
journeyctl init --path ~/Documents/MyObsidianVault --name my-journal --obsidian
```

## Supported Obsidian Plugins

Journey automatically detects and configures from these Obsidian plugins:

### Daily Notes (Core Plugin)
- Extracts date format, folder structure, and template settings
- Maps to Journey's `date_format`, `file_path_format`, and `template_file`

### Periodic Notes Plugin
- Extracts weekly, monthly, quarterly, and yearly format settings
- Stores periodic note formats in Journey's `phrases` configuration

### Journals Plugin
- Creates separate Journey vaults for each configured journal
- Each journal becomes a separate vault with its own folder structure and date format
- Supports multiple journals in a single Obsidian vault
- Uses `{{date:y}}` and `{{date:MM}}` variables for folder structure compatibility

## Multiple Journals Support

When using the Journals plugin, Journey creates one vault per journal:

```bash
# If your Obsidian vault has 3 journals: "Personal", "Work", "Daily"
journeyctl init --path ~/Documents/MyObsidianVault --obsidian

# This creates 3 Journey vaults:
# - MyObsidianVault-Personal
# - MyObsidianVault-Work  
# - MyObsidianVault-Daily
```

You can then use each vault independently:

```bash
# Add to Personal journal
journey --vault MyObsidianVault-Personal "Personal note"

# Add to Work journal
journey --vault MyObsidianVault-Work "Work note"

# Add to Daily journal
journey --vault MyObsidianVault-Daily "Daily note"
```

## Error Handling

Journey provides clear error messages for invalid Obsidian vaults:

```bash
# Path doesn't exist
journeyctl init --path /nonexistent/path --obsidian
# Error: Obsidian vault path does not exist: /nonexistent/path

# Path exists but isn't an Obsidian vault
journeyctl init --path /regular/folder --obsidian
# Error: Path is not a valid Obsidian vault (missing .obsidian directory): /regular/folder
```

## Configuration Extraction

Journey automatically extracts relevant settings from your Obsidian plugins:

**From Daily Notes:**
- Date format (e.g., `YYYY-MM-DD`)
- Folder structure (e.g., `/Daily Notes`)
- Template file (e.g., `Templates/Daily Note Template`)

**From Periodic Notes:**
- Weekly format (e.g., `YYYY-[W]ww`)
- Monthly format (e.g., `YYYY-MM`)
- Quarterly format (e.g., `YYYY-[Q]Q`)
- Yearly format (e.g., `YYYY`)

**From Journals Plugin:**
- Journal-specific folder structures
- Journal-specific date formats
- Multiple journal configurations

## Example Generated Configuration

After running `journeyctl init --path ~/MyObsidianVault --obsidian`, your `journey.yaml` might look like:

```yaml
vaults:
  MyObsidianVault-Personal:
    name: MyObsidianVault-Personal
    path: /home/user/MyObsidianVault
    locale: en_US.UTF-8
    date_format: YYYY-MM-DD
    file_path_format: Personal/{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md
    template_file: Templates/Personal Template
    
  MyObsidianVault-Work:
    name: MyObsidianVault-Work
    path: /home/user/MyObsidianVault
    locale: en_US.UTF-8
    date_format: YYYY-MM-DD
    file_path_format: Work/{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md
    template_file: Templates/Work Template
    
  MyObsidianVault-Daily:
    name: MyObsidianVault-Daily
    path: /home/user/MyObsidianVault
    locale: en_US.UTF-8
    date_format: YYYY-MM-DD
    file_path_format: 10-Journal/{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md
    template_file: Templates/Daily Note Template
    phrases:
      weekly_format: YYYY-[W]ww
      monthly_format: YYYY-MM
      quarterly_format: YYYY-[Q]Q
      yearly_format: YYYY
```

## Best Practices

1. **Backup First**: Always backup your Obsidian vault before initializing with Journey
2. **Test with Copy**: Consider testing with a copy of your vault first
3. **Review Configuration**: Check the generated `journey.yaml` to ensure settings are correct
4. **Template Compatibility**: Ensure your Obsidian templates use compatible variable syntax
5. **Path Consistency**: Use consistent path formats across your Obsidian and Journey configurations

