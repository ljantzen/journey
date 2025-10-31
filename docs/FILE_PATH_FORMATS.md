# Custom File Path Formats

Journey supports custom file path formats to organize your notes in any directory structure.

## Configuration

Set the `file_path_format` in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: /home/user/work-journal
    locale: en_US.UTF-8
    file_path_format: "work/{year}/{month:02}/{date:02}.md"
```

## Supported Placeholders

### Year Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{year}` | Full year | `2025` |
| `{date:y}` / `{{date:y}}` | Two-digit year | `25` |

### Month Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{month}` | Month number | `10`, `3` |
| `{month:02}` | Zero-padded month | `10`, `03` |
| `{date:MM}` / `{{date:MM}}` | Zero-padded month (Journals plugin) | `01`, `12` |
| `{Month}` | Full month name capitalized | `January`, `October` |
| `{month_name}` | Full month name lowercase | `january`, `october` |
| `{Month_short}` | Short month name capitalized | `Jan`, `Oct` |
| `{month_short}` | Short month name lowercase | `jan`, `oct` |

### Day Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{date}` | Day number | `24`, `5` |
| `{date:02}` | Zero-padded day | `24`, `05` |
| `{day}` | Alias for `{date}` | `24`, `5` |
| `{day:02}` | Zero-padded day | `24`, `05` |

### Weekday Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{Weekday}` | Full weekday capitalized | `Monday`, `Friday` |
| `{weekday}` | Full weekday lowercase | `monday`, `friday` |
| `{Weekday_short}` | Short weekday capitalized | `Mon`, `Fri` |
| `{weekday_short}` | Short weekday lowercase | `mon`, `fri` |

## Example Formats

### Basic Formats

```yaml
# Flat structure with ISO dates
file_path_format: "{year}-{month:02}-{date:02}.md"
# Output: 2025-10-24.md

# Year/Month hierarchy
file_path_format: "{year}/{month:02}/{date:02}.md"
# Output: 2025/10/24.md

# Named folders
file_path_format: "daily/{year}-{month:02}-{date:02}.md"
# Output: daily/2025-10-24.md
```

### Weekday-Based Organization

```yaml
# Organize by weekday
file_path_format: "notes/{Weekday}/{year}-{month:02}-{date:02}.md"
# Output: notes/Friday/2025-10-24.md

# Short weekday prefix
file_path_format: "daily/{weekday_short}_{year}-{month:02}-{date:02}.md"
# Output: daily/fri_2025-10-24.md

# Weekday folders with short prefix
file_path_format: "logs/{weekday}/{Weekday_short}_{year}-{month:02}-{date:02}.md"
# Output: logs/friday/Fri_2025-10-24.md
```

### Month-Based Organization

```yaml
# Full month name folders
file_path_format: "archives/{Month}/{year}-{month:02}-{date:02}.md"
# Output: archives/October/2025-10-24.md

# Month name with short prefix
file_path_format: "logs/{month_name}/{Month_short}_{year}-{month:02}-{date:02}.md"
# Output: logs/october/Oct_2025-10-24.md

# Year and month hierarchy
file_path_format: "{year}/{Month}/{date:02}.md"
# Output: 2025/October/24.md
```

### Obsidian Journals Plugin Compatibility

The Journals plugin uses `{{date:y}}` and `{{date:MM}}` syntax:

```yaml
# Two-digit year and month
file_path_format: "{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md"
# Output: 25/01/01-15.md

# With journal prefix
file_path_format: "Personal/{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md"
# Output: Personal/25/01/01-15.md

# Mixed syntax (both supported)
file_path_format: "journals/{date:y}/{date:MM}/{date:02}.md"
# Output: journals/25/01/15.md
```

## Default Behavior

If no `file_path_format` is specified, notes are stored as `YYYY-MM-DD.md` in the vault root directory:

```
vault-directory/
├── 2025-10-24.md
├── 2025-10-23.md
└── 2025-10-22.md
```

## Best Practices

### Organizing by Time Period

```yaml
# Daily notes organized by year and month
file_path_format: "{year}/{month:02}/{date:02}.md"

# Weekly organization (using weekday)
file_path_format: "{year}/Week-{month:02}/{Weekday}/{date:02}.md"

# Monthly archives
file_path_format: "{year}/{Month}/day-{date:02}.md"
```

### Project-Based Organization

```yaml
# Separate work and personal
work:
  file_path_format: "work/{year}/{month:02}/{date:02}.md"
  
personal:
  file_path_format: "personal/{year}/{month:02}/{date:02}.md"
```

### Obsidian Compatibility

```yaml
# Daily notes in Obsidian format
file_path_format: "Daily Notes/{year}-{month:02}-{date:02}.md"

# Journals plugin format
file_path_format: "10-Journal/{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md"

# Periodic notes structure
file_path_format: "Journal/{year}/{month:02} - {Month}/{date:02} {Weekday}.md"
```

## Directory Creation

Journey automatically creates any necessary directories in the path:

```yaml
file_path_format: "deep/nested/structure/{year}/{month:02}/{date:02}.md"
```

This will create:
```
vault-directory/
└── deep/
    └── nested/
        └── structure/
            └── 2025/
                └── 10/
                    └── 24.md
```

## Locale Considerations

Month and weekday names respect your vault's locale setting:

```yaml
vaults:
  norwegian:
    locale: no_NO.UTF-8
    file_path_format: "{year}/{Month}/{Weekday}-{date:02}.md"
    # Output: 2025/oktober/fredag-24.md
  
  english:
    locale: en_US.UTF-8
    file_path_format: "{year}/{Month}/{Weekday}-{date:02}.md"
    # Output: 2025/October/Friday-24.md
```

## Troubleshooting

### Invalid Path Characters
- Avoid special characters in literal parts of the path
- Use forward slashes `/` for path separators (works on all platforms)
- Avoid: `< > : " | ? *` in path components

### Path Not Created
- Check vault path permissions
- Verify placeholder syntax (correct braces)
- Ensure vault path exists and is writable

### Notes in Wrong Location
- Verify `file_path_format` configuration
- Check date placeholders are correct
- Test with a simple format first, then add complexity

## Migration

If you change `file_path_format`, existing notes remain in their original locations. Journey will:
- Read from old locations when listing/editing existing notes
- Create new notes in the new location

To migrate existing notes, you'll need to manually move them to match the new format.

