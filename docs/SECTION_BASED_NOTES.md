# Section-Based Notes

Journey supports organizing notes into specific sections within your daily markdown files. This feature allows you to categorize different types of notes and maintain a structured journal.

## Overview

When you configure a `section_name` in your vault configuration, Journey will:

1. **Add notes to the specified section** if it exists
2. **Create the section** if it doesn't exist (at the end of the file)
3. **Preserve other content** in the file

## Configuration

Set the `section_name` in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    locale: en_US.UTF-8
    section_name: "Daily Standup"  # Notes will be added to this section
    phrases: {}
    date_format: null
```

## How It Works

### 1. Section Exists
When the target section already exists in the file, new notes are added at the end of that section:

```markdown
---
date: 2025-10-24
---

# Other Section

- [09:00] Some other note

# Daily Standup

- [10:00] Morning standup notes
- [10:15] New note added here automatically  ← New note goes here

# Another Section

- [11:00] More notes
```

### 2. Section Doesn't Exist
When the target section doesn't exist, it will be created at the end of the file:

```markdown
---
date: 2025-10-24
---

# Other Section

- [09:00] Some other note

# Another Section

- [11:00] More notes

# Daily Standup

- [10:15] New note in new section  ← Section created with note
```

### 3. New File
When creating a new file, the section is created immediately:

```markdown
---
date: 2025-10-24
---

# Daily Standup

- [10:15] First note in new file
```

## Use Cases

### Work Journal
```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    section_name: "Daily Standup"
```

Perfect for daily standup notes, meeting notes, or project updates.

### Personal Journal
```yaml
vaults:
  personal:
    name: personal
    path: ~/Documents/personal-journal
    section_name: "Reflections"
```

Great for daily reflections, thoughts, or personal notes.

### Multi-Language Support
```yaml
vaults:
  european:
    name: european
    path: ~/Documents/european-journal
    locale: no_NO.UTF-8
    section_name: "Daglige Notater"  # Norwegian section name
```

Use section names in your preferred language.

## Advanced Usage

### Multiple Sections
You can have multiple sections in your markdown files. Journey will only add notes to the configured section:

```markdown
---
date: 2025-10-24
---

# Morning Notes

- [08:00] Early morning thoughts

# Daily Standup  ← Journey adds notes here

- [10:00] Standup notes
- [10:15] New note added here

# Evening Reflection

- [18:00] End of day thoughts
```

### Section Preservation
Journey preserves all existing content and sections. Only the target section is modified:

- ✅ Other sections remain untouched
- ✅ Frontmatter is preserved
- ✅ Existing notes in other sections are safe
- ✅ Only the target section gets new notes

## Examples

### Basic Usage
```bash
# Add a note to the "Daily Standup" section
journey --vault work Completed the project milestone

# Add a note with specific time
journey --vault work --time 14:30 Meeting with client went well
```

### With Date Override
```bash
# Add note to yesterday's standup section
journey --vault work --relative-date 1 Follow up on yesterday's action items
```

### Batch Input
```bash
# Add multiple notes to the section
echo -e "Morning standup\nClient meeting\nCode review" | journey --vault work --stdin
```

## Best Practices

1. **Choose descriptive section names** that clearly indicate the purpose
2. **Use consistent naming** across your vaults for similar purposes
3. **Consider your workflow** - choose sections that match your daily routine
4. **Test with a small vault** before applying to your main journal

## Troubleshooting

### Section Not Found
If you're not seeing notes in the expected section:
1. Check that `section_name` is correctly configured in your vault
2. Verify the section name matches exactly (case-sensitive)
3. Check if the section exists in your markdown file

### Notes Going to Wrong Place
If notes are being added to the end of the file instead of the section:
1. Ensure the section header uses `# Section Name` format
2. Check that the section name in the file matches your configuration
3. Verify there are no extra spaces or characters in the section name

### File Corruption
If your markdown file becomes corrupted:
1. Journey preserves existing content, but always backup important files
2. Use `journey --edit` to manually fix any issues
3. Consider using version control for your journal files
