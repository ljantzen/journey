# Template Files

Journey supports custom template files for new markdown files, allowing you to define consistent structures for your daily notes.

## Overview

When you configure a `template_file` in your vault configuration, Journey will:

1. **Use the template** when creating new markdown files
2. **Replace template variables** with actual values
3. **Handle note placement** based on template structure
4. **Fall back to default template** if no template is specified

## Configuration

Set the `template_file` in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    template_file: ~/Documents/templates/work-daily.md
    section_name: "Daily Standup"
    locale: en_US.UTF-8
    phrases: {}
    date_format: null
```

## Template Variables

Templates support the following variables that are automatically replaced:

### Date and Time Variables
- `{{date}}` - Date of the note being added (formatted according to vault settings)
- `{{time}}` - Time of the note being added (HH:MM:SS format)  
- `{{datetime}}` - Date and time of the note being added

**Important**: These variables reflect the date/time of the note being added, not the current date/time. When adding notes to different dates, the template variables will show the note's date/time, not when the template was processed.

### Content Variables
- `{{section_name}}` - The configured section name (if any)
- `{{note}}` - The note content (optional placeholder)

## Template Examples

### Basic Daily Template
```markdown
---
date: {{date}}
time: {{time}}
---

# {{section_name}}

## Morning
{{note}}

## Afternoon
- [ ] Task 1
- [ ] Task 2

## Evening
- Reflection: 
```

### Work Template
```markdown
---
date: {{date}}
time: {{time}}
---

# {{section_name}}

## Standup Notes
{{note}}

## Tasks
- [ ] Morning routine
- [ ] Work tasks
- [ ] Meetings

## Notes
- 
```

### Personal Template
```markdown
---
date: {{date}}
---

# {{section_name}}

## Today's Focus
{{note}}

## Gratitude
- 

## Learning
- 

## Tomorrow
- [ ] 
```

## Template Behavior

### Note Placement
Journey handles note placement in two ways:

1. **With `{{note}}` placeholder**: The note replaces the placeholder
2. **Without `{{note}}` placeholder**: The note is appended to the end of the file

### Variable Replacement
All template variables are replaced with actual values:
- Date/time variables use the vault's locale and format settings
- Section name uses the configured `section_name` from vault config
- Note content is the actual note being added

### Date/Time Behavior
Template variables always reflect the date/time of the note being added, not the current date/time:

```bash
# Adding a note to today (2025-10-24 14:30:00)
journey "Meeting notes"
# Template variables: {{date}} = 2025-10-24, {{time}} = 14:30:00

# Adding a note to yesterday (2025-10-23) with current time
journey --date 2025-10-23 "Yesterday's notes"  
# Template variables: {{date}} = 2025-10-23, {{time}} = 14:30:00 (current time)

# Adding a note to a specific date and time
journey --date 2025-10-20 --time 09:00 "Morning notes"
# Template variables: {{date}} = 2025-10-20, {{time}} = 09:00:00
```

This ensures that templates always show the correct date/time context for the note being added.

### Error Handling
- If template file doesn't exist, an error is returned
- If template file can't be read, an error is returned
- Missing variables are left as-is (not replaced)

## Use Cases

### Work Journal
```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    template_file: ~/Documents/templates/work-daily.md
    section_name: "Daily Standup"
```

Perfect for structured work notes with standup sections, task lists, and meeting notes.

### Personal Journal
```yaml
vaults:
  personal:
    name: personal
    path: ~/Documents/personal-journal
    template_file: ~/Documents/templates/personal-daily.md
    section_name: "Reflections"
```

Great for personal reflection, gratitude, and daily planning.

### Multi-Language Support
```yaml
vaults:
  european:
    name: european
    path: ~/Documents/european-journal
    locale: no_NO.UTF-8
    template_file: ~/Documents/templates/european-daily.md
    section_name: "Daglige Notater"
```

Use templates in your preferred language with localized section names.

## Advanced Usage

### Template with Multiple Sections
```markdown
---
date: {{date}}
---

# {{section_name}}

## Morning
{{note}}

## Work
- [ ] Task 1
- [ ] Task 2

## Personal
- [ ] Personal task 1
- [ ] Personal task 2

## Evening
- Reflection: 
```

### Template with Conditional Content
```markdown
---
date: {{date}}
---

# {{section_name}}

## Today's Notes
{{note}}

## Weather
- Temperature: 
- Conditions: 

## Mood
- Energy level: 
- Overall mood: 
```

## Best Practices

1. **Create template directory**: Organize templates in a dedicated folder
2. **Use descriptive names**: Name templates clearly (e.g., `work-daily.md`, `personal-weekly.md`)
3. **Test templates**: Verify template variables work as expected
4. **Keep templates simple**: Avoid overly complex structures
5. **Version control**: Keep templates in version control for consistency

## Troubleshooting

### Template Not Found
If you get "Failed to read template file" errors:
1. Check that the template file path is correct
2. Ensure the file exists and is readable
3. Use absolute paths if relative paths don't work

### Variables Not Replaced
If template variables aren't being replaced:
1. Check variable syntax (must be `{{variable}}`)
2. Ensure variables are supported (see list above)
3. Verify vault configuration is correct

### Notes in Wrong Place
If notes appear in unexpected locations:
1. Check if template has `{{note}}` placeholder
2. Without placeholder, notes are appended to end
3. With placeholder, notes replace the placeholder

## Examples

### Creating a Template
```bash
# Create template directory
mkdir -p ~/Documents/templates

# Create work template
cat > ~/Documents/templates/work-daily.md << 'EOF'
---
date: {{date}}
time: {{time}}
---

# {{section_name}}

## Standup
{{note}}

## Tasks
- [ ] Morning routine
- [ ] Work tasks

## Notes
- 
EOF

# Update vault configuration
journey --vault work --edit  # Edit config to add template_file
```

### Using Templates
```bash
# Add note - will use template for new files
journey --vault work Morning standup completed

# Template variables are automatically replaced
# {{date}} → 2025-10-24
# {{time}} → 10:30:00
# {{section_name}} → Daily Standup
# {{note}} → Morning standup completed
```

Templates provide a powerful way to structure your daily notes while maintaining consistency across your journal entries.
