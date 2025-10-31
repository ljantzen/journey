# Table Mode

Journey supports displaying notes in table format with customizable headers and locale-aware defaults.

## Configuration

Set the `list_type` to `table` in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    locale: en_US.UTF-8
    list_type: table  # or "bullet" (default)
```

## List Types

### Bullet Format (Default)

```yaml
list_type: bullet
```

Output:
```markdown
- 09:00:00 Morning standup meeting
- 10:30:00 Code review completed
- 14:00:00 Project planning session
```

### Table Format

```yaml
list_type: table
```

Output:
```markdown
| 09:00:00 | Morning standup meeting |
| 10:30:00 | Code review completed |
| 14:00:00 | Project planning session |
```

## Table Headers

When using table format, you can include markdown table headers using the `--header` flag:

```bash
# List notes with table headers
journey --list --header
```

### Output Behavior

**Without `--header` flag (default):**
```markdown
| 09:00:00 | Morning standup meeting |
| 10:30:00 | Code review completed |
| 14:00:00 | Project planning session |
```

**With `--header` flag:**
```markdown
| Time | Content |
|------|----------|
| 09:00:00 | Morning standup meeting |
| 10:30:00 | Code review completed |
| 14:00:00 | Project planning session |
```

### Table Mode Behavior

When listing notes in table format:
- The "Notes for: <date>" message is **automatically suppressed**
- Table headers are **not sorted** with note content
- Use `--header` flag to include markdown table headers
- Output is clean and ready for piping or copying

## Locale-Dependent Headers

Journey automatically uses appropriate table headers based on your vault's locale:

| Locale | Time Header | Content Header |
|--------|-------------|----------------|
| English (`en_US.UTF-8`) | Time | Content |
| Norwegian (`no_NO.UTF-8`) | Tid | Innhold |
| Swedish (`sv_SE.UTF-8`) | Tid | Innehåll |
| Danish (`da_DK.UTF-8`) | Tid | Indhold |
| Finnish (`fi_FI.UTF-8`) | Aika | Sisältö |
| German (`de_DE.UTF-8`) | Zeit | Inhalt |
| French (`fr_FR.UTF-8`) | Heure | Contenu |
| Spanish (`es_ES.UTF-8`) | Hora | Contenido |
| Italian (`it_IT.UTF-8`) | Ora | Contenuto |
| Dutch (`nl_NL.UTF-8`) | Tijd | Inhoud |
| Portuguese (`pt_PT.UTF-8`) | Hora | Conteúdo |
| Russian (`ru_RU.UTF-8`) | Время | Содержание |
| Japanese (`ja_JP.UTF-8`) | 時間 | 内容 |
| Chinese (`zh_CN.UTF-8`) | 时间 | 内容 |

### Example: Norwegian

```yaml
vaults:
  norwegian:
    locale: no_NO.UTF-8
    list_type: table
```

```bash
journey --list --header
```

Output:
```markdown
| Tid | Innhold |
|-----|----------|
| 09:00:00 | Morgen standup møte |
| 10:30:00 | Kode gjennomgang fullført |
```

### Example: German

```yaml
vaults:
  german:
    locale: de_DE.UTF-8
    list_type: table
```

```bash
journey --list --header
```

Output:
```markdown
| Zeit | Inhalt |
|------|--------|
| 09:00:00 | Morgen Standup Meeting |
| 10:30:00 | Code Review abgeschlossen |
```

## Custom Table Headers

You can override the default locale-dependent table headers:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    locale: en_US.UTF-8
    list_type: table
    table_headers:
      time: "Timestamp"
      content: "Note"
```

Output with `--header`:
```markdown
| Timestamp | Note |
|-----------|------|
| 09:00:00 | Morning standup meeting |
| 10:30:00 | Code review completed |
```

## Use Cases

### Clean Table Output
Perfect for piping to other tools or scripts:

```bash
# Export to file
journey --list --header > notes.md

# Copy to clipboard (Linux)
journey --list --header | xclip -selection clipboard

# Copy to clipboard (macOS)
journey --list --header | pbcopy

# Copy to clipboard (Windows)
journey --list --header | clip
```

### Markdown Compatibility
Headers make the output valid markdown tables:

```bash
# Create a report
echo "# Daily Report" > report.md
echo "" >> report.md
journey --list --header >> report.md
```

### Tool Integration
Use with tools that expect proper markdown table format:

```bash
# Convert to HTML
journey --list --header | pandoc -f markdown -t html

# Convert to PDF
journey --list --header | pandoc -f markdown -o notes.pdf
```

### Documentation
Include table headers when sharing or documenting notes:

```bash
# Share today's notes
journey --list --header | mail -s "Daily Notes" team@example.com
```

## Storage Format

Notes are stored in the markdown file in table format when `list_type: table` is configured:

```markdown
---
date: 2025-10-24
---

| 09:00:00 | Morning standup meeting |
| 10:30:00 | Code review completed |
| 14:00:00 | Project planning session |
```

## Mixing Formats

You can have different list types for different vaults:

```yaml
vaults:
  work:
    list_type: table  # Work notes in table format
  
  personal:
    list_type: bullet  # Personal notes as bullets
```

## Best Practices

### Use Tables for Structured Data
Tables work well when you need:
- Clean, structured output
- Easy scanning of time-based entries
- Export to other formats
- Integration with other tools

### Use Bullets for Quick Notes
Bullets work well when you need:
- Quick, informal notes
- Markdown compatibility without headers
- Simpler file structure
- Faster note-taking

### Custom Headers for Context
Use custom headers to match your workflow:

```yaml
# For meeting notes
table_headers:
  time: "When"
  content: "Discussion"

# For task tracking
table_headers:
  time: "Started"
  content: "Task"

# For time tracking
table_headers:
  time: "Duration"
  content: "Activity"
```

## Troubleshooting

### Headers Not Showing
- **Check flag**: Ensure you're using `--header` flag
- **Check list type**: Verify `list_type: table` is set
- **Check notes**: Ensure notes exist for the date

### Wrong Language Headers
- **Check locale**: Verify `locale` setting in vault configuration
- **Override**: Use custom `table_headers` if needed
- **System locale**: Ensure locale is installed on your system

### Mixed Format in File
- **Check configuration**: Verify `list_type` is set correctly
- **Migration**: Old notes may be in different format
- **Manual fix**: Use `journey --edit` to correct format

### Table Not Rendering
- **Check markdown**: Ensure proper table syntax
- **Check viewer**: Some markdown viewers require headers
- **Use --header**: Include headers for better compatibility

## Migration

### From Bullet to Table

If you change from bullet to table format, existing notes remain in bullet format. New notes will be added in table format. To convert existing notes:

1. **Manual conversion**: Use `journey --edit` to manually convert
2. **Script conversion**: Write a script to convert bullet points to table rows
3. **Keep mixed**: Both formats work, Journey handles both when listing

### From Table to Bullet

Similarly, changing from table to bullet format only affects new notes. Existing table notes remain as tables.

## Advanced Usage

### Filtering with grep

```bash
# Find specific content
journey --list | grep "meeting"

# Find by time range
journey --list | grep "^| 09:"

# Exclude certain content
journey --list | grep -v "standup"
```

### Sorting

```bash
# Sort by content (second column)
journey --list | sort -t'|' -k3

# Reverse chronological
journey --list | sort -r
```

### Formatting

```bash
# Add line numbers
journey --list --header | nl

# Add date prefix
echo "Date: $(date)" && journey --list --header
```

