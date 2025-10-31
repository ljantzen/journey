# Template Variables Reference

Journey supports custom template files for new markdown files with automatic variable substitution.

## Configuration

Set the `template_file` in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    template_file: ~/Documents/templates/work-daily.md
    section_header: "Daily Standup"
    locale: en_US.UTF-8
```

## Path Expansion

Template file paths support the same expansion as vault paths:

**Unix/Linux/macOS:**
```yaml
template_file: "~/Documents/templates/work-daily.md"
# Expands to: /home/username/Documents/templates/work-daily.md
```

**Windows:**
```yaml
template_file: "%USERPROFILE%/Documents/templates/work-daily.md"
# Expands to: C:\Users\username\Documents\templates\work-daily.md

# Multiple variables
template_file: "%USERPROFILE%/Documents/%USERNAME%_templates/journal.md"
```

## Available Variables

Both single `{variable}` and double `{{variable}}` brace formats are supported for compatibility.

### Date and Time Variables

| Variable | Description | Example Output |
|----------|-------------|----------------|
| `{{date}}` / `{date}` | Date of the note | `2025-10-24` |
| `{{time}}` / `{time}` | Time of the note | `14:30:00` |
| `{{datetime}}` / `{datetime}` | Date and time | `2025-10-24 14:30:00` |
| `{{created}}` / `{created}` | Full timestamp | `2025-10-24 14:30:00` |
| `{{today}}` / `{today}` | Today's date | `2025-10-24` |

### Relative Date Variables

| Variable | Description | Example Output |
|----------|-------------|----------------|
| `{{yesterday}}` / `{yesterday}` | Yesterday's date | `2025-10-23` |
| `{{tomorrow}}` / `{tomorrow}` | Tomorrow's date | `2025-10-25` |

### Weekday Variables

| Variable | Description | Example Output |
|----------|-------------|----------------|
| `{{weekday}}` / `{weekday}` | Full weekday name | `Monday` |
| `{{Weekday}}` / `{Weekday}` | Abbreviated weekday | `Mon` |

### Other Variables

| Variable | Description | Example Output |
|----------|-------------|----------------|
| `{{section_header}}` / `{section_header}` | Configured section header | `Daily Standup` |
| `{{note}}` / `{note}` | Note content placeholder | *(replaced with actual note)* |

## Important Notes

- **Date Context**: Template variables reflect the date/time of the note being added, not the current date/time
- **Backward Compatibility**: Both `{{section_name}}` and `{{section_header}}` are supported
- **Note Placeholder**: If `{{note}}` is not found in the template, the note is appended to the end of the file
- **Missing Variables**: Undefined variables are left unchanged in the output

## Example Template

```markdown
---
created: {created}
updated: {created}
---

[[{yesterday}]] [[{tomorrow}]]

## 📅️ {today} {weekday}

## 🎯 Goals

## 🕗 Schedule

## 🔨 Tasks

## 👀️ Notes

{note}
```

### Output Example

When adding a note on Monday, October 24, 2025 at 14:30:00:

```markdown
---
created: 2025-10-24 14:30:00
updated: 2025-10-24 14:30:00
---

[[2025-10-23]] [[2025-10-25]]

## 📅️ 2025-10-24 Monday

## 🎯 Goals

## 🕗 Schedule

## 🔨 Tasks

## 👀️ Notes

- 14:30:00 Your note content here
```

## Template Behavior

1. **New Files**: If the daily note file doesn't exist, it's created from the template
2. **Existing Files**: If the file exists, notes are appended (template is not reapplied)
3. **Note Insertion**: 
   - If `{{note}}` placeholder exists, it's replaced with the note content
   - If no placeholder, the note is appended to the end of the file
4. **Section Headers**: If `section_header` is configured, notes are added to that section
5. **Variable Substitution**: All template variables are replaced during file creation

## Advanced Usage

### Multiple Section Headers

Combine templates with category-specific section headers:

```yaml
vaults:
  work:
    template_file: ~/templates/work.md
    section_header: "General Notes"
    section_header_work: "Work Notes"
    section_header_personal: "Personal Notes"
    section_header_health: "Health & Fitness"
```

Template:
```markdown
---
date: {date}
---

# {date} - {weekday}

## Work Notes

## Personal Notes

## Health & Fitness

## General Notes

{note}
```

### Dynamic Section Headers

Use the `{{section_header}}` variable to dynamically insert the active section:

```markdown
---
date: {date}
---

# Daily Log - {weekday}, {date}

## {section_header}

{note}
```

When adding with `-c work`, the section header will be "Work Notes" (if configured).

## Troubleshooting

### Template Not Applied
- **Check path**: Ensure `template_file` path is correct and file exists
- **Check permissions**: Ensure the template file is readable
- **Check syntax**: Verify template variables use correct syntax

### Variables Not Replaced
- **Check spelling**: Variable names are case-sensitive
- **Check braces**: Use either `{var}` or `{{var}}`, not `{{{var}}}`
- **Check support**: Ensure the variable is in the supported list above

### Notes in Wrong Location
- **Section header**: Check `section_header` configuration
- **Note placeholder**: Verify `{{note}}` placement in template
- **Existing files**: Templates only apply to new files, not existing ones

