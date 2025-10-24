# Journey - A CLI-based Journal Application

Journey is a command-line journal application written in Rust that allows you to manage daily notes with automatic timestamping and flexible date/time handling.

## Features

- **Vault Management**: Create and manage multiple journal vaults
- **Automatic Timestamping**: Notes are automatically timestamped
- **Flexible Date/Time**: Support for absolute dates, relative dates, and custom times
- **Markdown Storage**: Notes are stored as markdown files with frontmatter
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Configuration**: YAML-based configuration stored in platform-appropriate locations

## Installation

```bash
git clone <repository-url>
cd journey
cargo build --release
```

## Quick Start

1. **Initialize a vault**:
   ```bash
   journey init --path ~/my-journal --name personal
   ```

2. **Add a note**:
   ```bash
   journey "This is my first note"
   ```

3. **List today's notes**:
   ```bash
   journey --list
   ```

4. **Add a note for yesterday**:
   ```bash
   journey --relative-date 1 "Note for yesterday"
   ```

## Commands

### Initialize a Vault
```bash
journey init --path /path/to/vault --name vault-name
```

### Add Notes
```bash
# Add note for today (default behavior)
journey My note content

# Add note with explicit command
journey add My note content

# Add note for specific date (no quotes needed)
journey --date 2025-10-22 Note for specific date
journey --date 2025-12-25 Christmas planning note

# Different date formats work automatically based on locale
journey --date 10/24/2025 US format date
journey --date 24.10.2025 European format date
journey --date October 24, 2025 Long format date

# Relative dates
journey --relative-date -7 Note for next week
journey --relative-date 1 Note for yesterday

# Add note for yesterday (no quotes needed)
journey --relative-date 1 Note for yesterday

# Add note for tomorrow (no quotes needed)
journey --relative-date -1 Note for tomorrow

# Add note for next week (no quotes needed)
journey --relative-date -7 Note for next week

# Add note with specific time (no quotes needed)
journey --time 14:30 Note with specific time

# Read from stdin (each line becomes a separate note)
echo -e "First note\nSecond note\nThird note" | journey --stdin

# Read from file
cat notes.txt | journey --stdin

# Force 12-hour format parsing (compact format, no quotes needed)
journey --time 2:30PM --time-format 12h Note with 12h format

# Force 24-hour format parsing (no quotes needed)
journey --time 14:30 --time-format 24h Note with 24h format
```

### List Notes
```bash
# List today's notes
journey --list

# List notes for specific date
journey --date 2025-10-22 --list

# List notes for yesterday
journey --relative-date 1 --list
```

### Edit Notes
```bash
# Edit today's notes
journey --edit

# Edit notes for specific date
journey --date 2025-10-22 --edit
```

### Section-Based Notes

Journey supports organizing notes into specific sections within your daily markdown files. This is useful for categorizing different types of notes (e.g., work vs personal, different projects, etc.).

#### Configuration
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

#### How It Works
- When `section_name` is configured, new notes are automatically added to that section
- If the section doesn't exist, it will be created at the end of the file
- If the section exists, notes are added at the end of that section
- Other sections and content in the file are preserved

#### Example Markdown Structure
```markdown
---
date: 2025-10-24
---

# Other Section

- [09:00] Some other note

# Daily Standup

- [10:00] Morning standup notes
- [10:15] New note added here automatically

# Another Section

- [11:00] More notes
```

### Template Files

Journey supports custom template files for new markdown files. This allows you to define a consistent structure for your daily notes.

#### Configuration
Set the `template_file` in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: ~/Documents/work-journal
    template_file: ~/Documents/templates/work-daily.md  # Template for new files
    section_name: "Daily Standup"
    locale: en_US.UTF-8
    phrases: {}
    date_format: null
```

#### Template Variables
Templates support the following variables that are automatically replaced:

- `{{date}}` - Date of the note being added (formatted according to vault settings)
- `{{time}}` - Time of the note being added (HH:MM:SS format)
- `{{datetime}}` - Date and time of the note being added
- `{{section_name}}` - The configured section name (if any)
- `{{note}}` - The note content (optional placeholder)

**Important**: Template variables reflect the date/time of the note being added, not the current date/time. This means when adding notes to different dates, the template variables will show the note's date/time, not when the template was processed.

#### Example Template
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

#### Template Behavior
- If `template_file` is specified, it will be used for new files
- If `template_file` is not specified, the default template is used
- Template variables are replaced with actual values
- If `{{note}}` placeholder is not found, the note is appended to the end
- If `{{note}}` placeholder is found, it's replaced with the note content

## Configuration

The configuration file is stored at:
- **Linux**: `~/.config/journey/journey.yaml`
- **macOS**: `~/Library/Application Support/journey/journey.yaml`
- **Windows**: `%APPDATA%\journey\journey.yaml`

### Configuration Structure

```yaml
vaults:
  personal:
    name: personal
    path: /home/user/journal
    locale: en_US.UTF-8
    phrases: {}
    section_name: null
```

### Phrase Expansion

Journey supports custom phrase expansion to make note-taking faster. Define phrases in your vault configuration:

```yaml
vaults:
  work:
    name: work
    path: /home/user/work-journal
    locale: en_US.UTF-8
    phrases:
      "@meeting": "Team meeting about project status"
      "@lunch": "Had lunch at the usual place"
      "@code": "Coding session on main project"
      "@review": "Code review completed"
    section_name: "Daily Standup"
```

When you add a note containing a phrase key, it gets automatically replaced with the corresponding value:

```bash
# This note:
journey "@meeting went well, then @lunch"

# Becomes this in your journal:
# - [14:30:00] Team meeting about project status went well, then Had lunch at the usual place
```

**Phrase Features:**
- **Longest match first**: If you have `@work` and `@workout`, typing `@workout` will match the longer phrase
- **Multiple phrases**: You can use multiple phrases in a single note
- **Case sensitive**: Phrases are matched exactly as defined
- **Global replacement**: All occurrences of a phrase in a note are replaced

## File Structure

Notes are by default stored as markdown files with the following structure:

```
vault-directory/
├── 2025-10-24.md
├── 2025-10-23.md
└── 2025-10-22.md
```

Each markdown file contains by default:
```markdown
---
date: 2025-10-24
---

- [13:35:27] This is my first note
- [13:35:35] This is my second note
```

This can be changed by using templates, see further down. 


### Custom File Path Formats

Journey supports custom file path formats to organize your notes in any directory structure. Here is an example:

```yaml
vaults:
  work:
    name: work
    path: /home/user/work-journal
    locale: en_US.UTF-8
    file_path_format: "work/{year}/{month:02}/{date:02}.md"
```

**Supported Placeholders:**
- `{year}` - Full year (e.g., 2025)
- `{month}` - Month number (e.g., 10)
- `{month:02}` - Zero-padded month (e.g., 10, 03)
- `{date}` - Day number (e.g., 24)
- `{date:02}` - Zero-padded day (e.g., 24, 05)
- `{day}` - Alias for `{date}`
- `{Weekday}` - Full weekday name capitalized (e.g., Monday, Tuesday)
- `{weekday}` - Full weekday name lowercase (e.g., monday, tuesday)
- `{Weekday_short}` - Short weekday name capitalized (e.g., Mon, Tue)
- `{weekday_short}` - Short weekday name lowercase (e.g., mon, tue)
- `{Month}` - Full month name capitalized (e.g., January, February)
- `{month_name}` - Full month name lowercase (e.g., january, february)
- `{Month_short}` - Short month name capitalized (e.g., Jan, Feb)
- `{month_short}` - Short month name lowercase (e.g., jan, feb)

**Example Formats:**
- `"work/{year}/{month:02}/{date:02}.md"` → `work/2025/10/24.md`
- `"journals/{year}/{month}/{day}.md"` → `journals/2025/10/24.md`
- `"daily/{year}-{month:02}-{date:02}.md"` → `daily/2025-10-24.md`
- `"notes/{Weekday}/{year}-{month:02}-{date:02}.md"` → `notes/Friday/2025-10-24.md`
- `"daily/{weekday_short}_{year}-{month:02}-{date:02}.md"` → `daily/fri_2025-10-24.md`
- `"logs/{weekday}/{Weekday_short}_{year}-{month:02}-{date:02}.md"` → `logs/friday/Fri_2025-10-24.md`
- `"archives/{Month}/{year}-{month:02}-{date:02}.md"` → `archives/October/2025-10-24.md`
- `"logs/{month_name}/{Month_short}_{year}-{month:02}-{date:02}.md"` → `logs/october/Oct_2025-10-24.md`

**Default Behavior:**
If no `file_path_format` is specified, notes are stored as `YYYY-MM-DD.md` in the vault root directory.


## Options

- `-d, --date <DATE>`: Specify date (supports multiple formats, see below)
- `-r, --relative-date <DAYS>`: Days offset (positive = past, negative = future, 0 = today)
- `-t, --time <TIME>`: Specify time in HH:MM or HH:MM:SS format
- `--time-format <FORMAT>`: Override time format (12h|24h)
- `--stdin`: Read input from stdin (each line becomes a separate note)
- `-V, --vault <NAME>`: Specify vault name
- `-v, --version`: Show version information
- `-l, --list`: List notes
- `-e, --edit`: Edit notes
- `-a, --add-note <NOTE>`: Add a note

### Date Format Support

The `--date` flag supports multiple date formats based on your locale:

#### English/US Locale (`en_US.UTF-8`)
- `2025-10-24` (ISO format)
- `10/24/2025` (US format)
- `10-24-2025` (US with dashes)
- `October 24, 2025` (Long format)
- `Oct 24, 2025` (Short format)

#### Norwegian Locale (`no_NO.UTF-8`)
- `2025-10-24` (ISO format)
- `24.10.2025` (Norwegian format)
- `24/10/2025` (European format)
- `24-10-2025` (European with dashes)
- `24. oktober 2025` (Norwegian long)
- `24. okt 2025` (Norwegian short)

#### Custom Date Format Override
You can also specify a custom date format in your `journey.yaml`:

```yaml
vaults:
  my_vault:
    date_format: "DD.MM.YYYY"  # European format
    # or
    date_format: "MM/DD/YYYY"  # US format
    # or any chrono format string
```

### Relative Date Examples

The `--relative-date` flag uses intuitive numbering:

- `journey --relative-date 1` → Yesterday (1 day ago)
- `journey --relative-date 7` → Last week (7 days ago)
- `journey --relative-date 0` → Today
- `journey --relative-date -1` → Tomorrow (1 day in the future)
- `journey --relative-date -7` → Next week (7 days in the future)

## Examples

```bash
# Create a vault
journey init --path ~/journal --name daily

# Add notes throughout the day (no quotes needed)
journey Morning coffee and planning
journey Completed the project milestone
journey Evening reflection on the day

# Add a note for yesterday (no quotes needed)
journey --relative-date 1 Forgot to log this yesterday

# List all notes for today
journey --list

# Edit today's notes
journey --edit

# Add a note with specific time (no quotes needed)
journey --time 09:30 "Early morning meeting notes"
```

## Time Format Override

By default, Journey automatically detects and supports both 12-hour and 24-hour time formats based on your locale. However, you can override this behavior to force a specific format:

### 12-Hour Format Override
```bash
# Force 12-hour format parsing (compact format, no quotes needed)
journey --time 2:30PM --time-format 12h Meeting at 2:30 PM
journey --time 2:30:45PM --time-format 12h Precise time
journey --time 2:30PM --time-format 12h Compact format
```

### 24-Hour Format Override
```bash
# Force 24-hour format parsing (no quotes needed)
journey --time 14:30 --time-format 24h Meeting at 14:30
journey --time 14:30:45 --time-format 24h Precise time
```

### Use Cases
- **Consistency**: Force a specific format across different locales
- **Validation**: Ensure only certain time formats are accepted
- **Integration**: Match external systems that use specific time formats

### Error Handling
If you specify a time format override that doesn't match the time string, Journey will return an error:
```bash
# This will fail - 12h time with 24h format override
journey --time 2:30PM --time-format 24h This will fail

# This will fail - 24h time with 12h format override  
journey --time 14:30 --time-format 12h This will fail
```

## Stdin Support

Journey supports reading input from stdin, making it easy to pipe content from other commands or read from files. Each line of input becomes a separate note.

### Basic Stdin Usage
```bash
# Read from stdin (each line becomes a separate note)
echo -e "Meeting notes\nTask completed\nFollow up needed" | journey --stdin

# Read from file
cat meeting-notes.txt | journey --stdin

# Pipe from other commands
ls -la | head -5 | journey --stdin
```

### Stdin with Time/Date Overrides
```bash
# All notes get the same timestamp
echo -e "Morning standup\nCode review\nDeployment" | journey --stdin --time 09:00

# All notes get the same date
echo -e "Weekend tasks\nPersonal notes" | journey --stdin --date 2025-10-25
```

### Use Cases
- **Batch Processing**: Import multiple notes from files
- **Command Integration**: Pipe output from other tools
- **Automation**: Script-based note creation
- **Data Migration**: Import notes from other systems

### Examples
```bash
# Import from a text file
cat daily-tasks.txt | journey --stdin

# Import from a CSV (extract specific column)
cut -d',' -f1 data.csv | journey --stdin

# Import from a log file
tail -n 10 app.log | journey --stdin

# Import with specific time
echo "Daily standup notes" | journey --stdin --time 09:00
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- --help
```
