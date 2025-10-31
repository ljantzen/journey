# Categories and Section Headers

Journey supports organizing notes into specific sections within your daily markdown files using categories.

## Basic Configuration

Set the `section_header` in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    locale: en_US.UTF-8
    section_header: "Daily Log"  # Default section for notes
```

## Category-Specific Sections

Journey supports category-specific section headers using the `-c/--category` flag:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    locale: en_US.UTF-8
    section_header: "General Notes"  # Default section
    section_header_work: "Work Notes"
    section_header_personal: "Personal Notes"
    section_header_health: "Health & Fitness"
    section_header_meetings: "Meeting Notes"
```

### Using Categories

```bash
# Add a work note to the "Work Notes" section
journey -c work "Completed the quarterly report"

# Add a personal note to the "Personal Notes" section  
journey -c personal "Had dinner with friends"

# Add a health note to the "Health & Fitness" section
journey -c health "30 minutes of cardio"

# Add a note to the default section (no category)
journey "General observation"
```

## Supported Category Headers

Journey recognizes these category-specific section headers:

| Category | Configuration Field | Example Value |
|----------|-------------------|---------------|
| `work` | `section_header_work` | "Work Notes" |
| `personal` | `section_header_personal` | "Personal Notes" |
| `health` | `section_header_health` | "Health & Fitness" |
| `meetings` | `section_header_meetings` | "Meeting Notes" |
| *(default)* | `section_header` | "Daily Log" |

## How It Works

- When a section header is configured, new notes are automatically added to that section
- If the section doesn't exist, it will be created at the end of the file
- If the section exists, notes are added at the end of that section
- Other sections and content in the file are preserved
- Category-specific headers take precedence over the default `section_header`

## Example Markdown Structure

```markdown
---
date: 2025-10-24
---

# General Notes

- 08:00:00 Morning coffee and planning

# Work Notes

- 09:00:00 Daily standup meeting
- 10:00:00 Completed the quarterly report

# Personal Notes

- 12:00:00 Had dinner with friends

# Health & Fitness

- 18:00:00 30 minutes of cardio

# Meeting Notes

- 14:00:00 Project planning session
```

## Use Cases

### Work Journal with Categories

```yaml
vaults:
  work:
    section_header: "Daily Log"
    section_header_work: "Work Tasks"
    section_header_meetings: "Meetings"
    section_header_personal: "Personal"
```

Usage:
```bash
journey -c work "Fixed critical bug in production"
journey -c meetings "Sprint planning with team"
journey -c personal "Lunch with colleague"
journey "General note"  # Goes to "Daily Log"
```

### Personal Journal

```yaml
vaults:
  personal:
    section_header: "Daily Reflections"
    section_header_health: "Health & Wellness"
    section_header_personal: "Personal Goals"
```

### Project Notes

```yaml
vaults:
  project:
    section_header: "Project Updates"
    section_header_work: "Development"
    section_header_meetings: "Standups"
```

## Listing Notes by Category

You can list notes from a specific category:

```bash
# List all work notes for today
journey --list -c work

# List personal notes for a specific date
journey --list --date 2025-10-24 -c personal

# List health notes for yesterday
journey --list --relative-date 1 -c health
```

## Template Integration

Section headers work seamlessly with template files. Use the `{{section_header}}` variable in your template:

```markdown
---
date: {date}
---

# Work Notes

# Personal Notes

# Health & Fitness

# {{section_header}}

{note}
```

When creating a new file, the section header will be automatically inserted based on the category used.

## Backward Compatibility

Journey maintains backward compatibility with the legacy `section_name` field:

```yaml
vaults:
  legacy:
    section_name: "Daily Standup"  # Still supported
```

If both `section_header` and `section_name` are present, `section_header` takes precedence. If only `section_name` is present, it will be used as the default section header.

## Best Practices

### Consistent Section Names
Use consistent section names across your vaults:

```yaml
vaults:
  work:
    section_header_work: "Work Notes"
    section_header_personal: "Personal Notes"
  
  personal:
    section_header_work: "Work Notes"  # Same names
    section_header_personal: "Personal Notes"
```

### Descriptive Names
Use clear, descriptive section names:

```yaml
# Good
section_header_work: "Work Tasks"
section_header_meetings: "Meeting Notes"

# Less clear
section_header_work: "W"
section_header_meetings: "M"
```

### Pre-create Sections in Templates
Include all section headers in your template to maintain consistent structure:

```markdown
---
date: {date}
---

# Work Notes

# Personal Notes

# Health & Fitness

# Meeting Notes

# Daily Log
```

### Use Default Section
Always configure a default `section_header` for notes without a category:

```yaml
vaults:
  work:
    section_header: "General Notes"  # Default
    section_header_work: "Work Notes"
    section_header_personal: "Personal Notes"
```

## Combining Categories with Other Features

### Categories with Phrases

```bash
# Combine category and phrase expansion
journey -c work "@standup discussed project status"
# Output in "Work Notes": - 09:00:00 Daily standup discussed project status
```

### Categories with Dates

```bash
# Add categorized note for yesterday
journey -c work --relative-date 1 "Forgot to log meeting"
```

### Categories with Custom Times

```bash
# Add categorized note with specific time
journey -c meetings --time 14:00 "Project planning session"
```

## Troubleshooting

### Notes Not in Correct Section
- **Check category**: Verify you're using the correct `-c` flag
- **Check configuration**: Ensure the category-specific header is configured
- **Check spelling**: Section headers are case-sensitive

### Section Not Created
- **Check permissions**: Ensure the file is writable
- **Check section header**: Verify the section header is configured
- **Check file format**: Ensure the file is valid markdown

### Multiple Sections with Same Name
- Journey adds notes to the first matching section
- Avoid duplicate section headers in your markdown files
- Use unique section names in your configuration

### Category Not Working
- **Check vault**: Ensure you're using the correct vault with `--vault`
- **Check configuration**: Verify category-specific header is defined
- **Check syntax**: Use lowercase category names (`work`, not `Work`)

