# Journey - A CLI-based Journal Application

Journey is a command-line journal application written in Rust that allows you to manage daily notes with automatic timestamping and flexible date/time handling.

## Features

- **Vault Management**: Create and manage multiple journal vaults
- **Obsidian Integration**: Import configuration from existing Obsidian vaults and plugins
- **Automatic Timestamping**: Notes are automatically timestamped
- **Flexible Date/Time**: Support for absolute dates, relative dates, and custom times
- **Markdown Storage**: Notes are stored as markdown files with frontmatter
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Configuration**: YAML-based configuration stored in platform-appropriate locations
- **Dual Binary Architecture**: Separate `journey` and `journeyctl` commands for different operations

# License 

This software is licensed under a combined MIT and SPPL license.  It is basically a MIT license, but in order to be compliant you need to send me a postcard.  Details in [LICENSE](https://github.com/ljantzen/journey/blob/main/LICENSE)

## Installation

You can install journey using cargo:

```bash
cargo install journey-cli
```

Or build from source:

```bash
git clone https://github.com/ljantzen/journey.git
cd journey
cargo build --release
cargo install --path .
```

Or download binaries directly from the [github releases page](https://github.com/ljantzen/journey/releases).

## Binary Architecture

Journey uses a dual binary architecture for better separation of concerns:

- **`journey`**: Main journal application for note operations (add, list, edit)
- **`journeyctl`**: Vault management tool for administrative operations (init, default vault management)

This separation allows for:
- Cleaner command interfaces
- Better organization of functionality
- Easier maintenance and development

### Quick Command Reference

**Journey (Note Operations):**
```bash
journey "My note"                    # Add note
journey --list                       # List notes
journey --edit                       # Edit notes
```

**Journeyctl (Vault Management):**
```bash
journeyctl init --path ~/journal     # Initialize vault
journeyctl init --path ~/obsidian-vault --obsidian  # Initialize from Obsidian vault
journeyctl list                      # List all vaults
journeyctl set-default vault-name    # Set default vault
journeyctl show-default              # Show current default
journeyctl unset-default             # Remove default vault
journeyctl unlist-vault vault-name  # Unlist a vault
```

## Quick Start

1. **Initialize a vault**:
   ```bash
   # Regular vault (Unix/Linux/macOS)
   journeyctl init --path ~/my-journal --name personal
   
   # Regular vault (Windows)
   journeyctl init --path "%USERPROFILE%\my-journal" --name personal
   
   # Initialize from existing Obsidian vault
   journeyctl init --path ~/Documents/MyObsidianVault --obsidian
   ```

2. **Add a note**:
   ```bash
   journey This is my first note
   ```

3. **List today's notes**:
   ```bash
   journey --list
   # Or simply:
   journey
   ```

4. **Add a note for yesterday**:
   ```bash
   journey --relative-date 1 Note for yesterday
   ```
   The timestamp will be the current time, just 24 hours before.

## Commands

### Default Behavior

When you run `journey` without any arguments, it lists today's notes (equivalent to `journey --list`):

```bash
# List today's notes (default behavior)
journey

# Explicitly list notes (same as above)
journey --list
```

This provides a quick way to see your notes without needing to remember flags.

### Initialize a Vault

```bash
journeyctl init --path /path/to/vault --name vault-name
```

### Obsidian Integration

Journey can integrate with existing Obsidian vaults, automatically detecting and configuring from your Obsidian plugins.

#### Basic Obsidian Vault Initialization

```bash
# Initialize from an existing Obsidian vault
journeyctl init --path ~/Documents/MyObsidianVault --obsidian

# With custom name
journeyctl init --path ~/Documents/MyObsidianVault --name my-journal --obsidian
```

#### Supported Obsidian Plugins

Journey automatically detects and configures from these Obsidian plugins:

**Daily Notes (Core Plugin)**
- Extracts date format, folder structure, and template settings
- Maps to Journey's `date_format`, `file_path_format`, and `template_file`

**Periodic Notes Plugin**
- Extracts weekly, monthly, quarterly, and yearly format settings
- Stores periodic note formats in Journey's `phrases` configuration

**Journals Plugin**
- Creates separate Journey vaults for each configured journal
- Each journal becomes a separate vault with its own folder structure and date format
- Supports multiple journals in a single Obsidian vault
- Uses `{{date:y}}` and `{{date:MM}}` variables for folder structure compatibility

#### Multiple Journals Support

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

#### Error Handling

Journey provides clear error messages for invalid Obsidian vaults:

```bash
# Path doesn't exist
journeyctl init --path /nonexistent/path --obsidian
# Error: Obsidian vault path does not exist: /nonexistent/path

# Path exists but isn't an Obsidian vault
journeyctl init --path /regular/folder --obsidian
# Error: Path is not a valid Obsidian vault (missing .obsidian directory): /regular/folder
```

#### Configuration Extraction

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

#### Example Generated Configuration

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

### Vault Management

Journey provides comprehensive vault management through `journeyctl`:

```bash
# List all configured vaults
journeyctl list

# Set a default vault
journeyctl set-default vault-name

# Show current default vault
journeyctl show-default

# Unset the default vault
journeyctl unset-default

# Unlist a vault
journeyctl unlist-vault vault-name
```

**Benefits:**
- No need to specify `--vault` for most operations
- Commands like `journey "My note"` will automatically use the default vault
- Still works with `--vault` to override the default when needed

### Unlist a Vault

```bash
journeyctl unlist-vault vault-name
```

**Important Notes:**
- **This only removes the vault from Journey's configuration** - it does NOT delete the actual files
- **Safe operation** - your notes and files remain untouched
- **Default vault handling** - if you delete the default vault, the default is automatically cleared
- **Error handling** - provides clear error messages for non-existent vaults

**Examples:**
```bash
# Unlist a specific vault
journeyctl unlist-vault my-old-vault

# Unlist the default vault (default will be cleared)
journeyctl unlist-vault default-vault

# Error: vault doesn't exist
journeyctl unlist-vault non-existent
# Error: Vault not found: Vault 'non-existent' not found
```

**What happens when you unlist a vault:**
1. Vault is removed from Journey's configuration
2. If the unlisted vault was the default, the default is cleared
3. Your actual files and notes remain completely untouched
4. You can re-add the vault later using `journeyctl init` if needed

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

**Template File Path Expansion:**
Template file paths support the same expansion as vault paths:

- **Unix/Linux/macOS**: `~/templates/journal.md` → `/home/username/templates/journal.md`
- **Windows**: `"%USERPROFILE%/templates/journal.md"` → `C:\Users\username\templates\journal.md`

**Examples:**
```yaml
# Unix/Linux/macOS
template_file: "~/Documents/templates/work-daily.md"

# Windows
template_file: "%USERPROFILE%/Documents/templates/work-daily.md"

# Windows with multiple variables
template_file: "%USERPROFILE%/Documents/%USERNAME%_templates/journal.md"
```

#### Template Variables
Templates support the following variables that are automatically replaced:

**Date and Time Variables:**
- `{{date}}` / `{date}` - Date of the note being added (formatted according to vault settings)
- `{{time}}` / `{time}` - Time of the note being added (HH:MM:SS format)
- `{{datetime}}` / `{datetime}` - Date and time of the note being added
- `{{created}}` / `{created}` - Full timestamp when the note was created (YYYY-MM-DD HH:MM:SS)
- `{{today}}` / `{today}` - Today's date (same as date, for clarity)

**Relative Date Variables:**
- `{{yesterday}}` / `{yesterday}` - Yesterday's date (same format as note filenames)
- `{{tomorrow}}` / `{tomorrow}` - Tomorrow's date (same format as note filenames)

**Weekday Variables:**
- `{{weekday}}` / `{weekday}` - Full weekday name (Monday, Tuesday, etc.)
- `{{Weekday}}` / `{Weekday}` - Abbreviated weekday name (Mon, Tue, etc.)

**Other Variables:**
- `{{section_name}}` / `{section_name}` - The configured section name (if any)
- `{{note}}` / `{note}` - The note content (optional placeholder)

**Note**: Both single `{variable}` and double `{{variable}}` brace formats are supported for compatibility.

**Important**: Template variables reflect the date/time of the note being added, not the current date/time. This means when adding notes to different dates, the template variables will show the note's date/time, not when the template was processed.

#### Example Template
```markdown
---
created: {created}
updated: {created}
---

[[{yesterday}]] [[{tomorrow}]]

## 📅️ {today} {weekday}

## 🎯

## 🕗

## 🔨

## 👀️

{note}
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

### Path Expansion

Journey supports automatic path expansion for vault directories, making configuration more portable and user-friendly.

#### Tilde Expansion (Unix/Linux/macOS)
Use `~` to reference your home directory:

```yaml
vaults:
  personal:
    name: personal
    path: ~/Documents/journal  # Expands to /home/username/Documents/journal
    locale: en_US.UTF-8
    phrases: {}
    section_name: null
```

#### Windows Environment Variables
On Windows, you can use environment variables in paths:

```yaml
vaults:
  personal:
    name: personal
    path: "%USERPROFILE%/Documents/journal"  # Expands to C:\Users\username\Documents\journal
    locale: en_US.UTF-8
    phrases: {}
    section_name: null
```

**Supported Windows Environment Variables:**
- `%USERPROFILE%` - User's home directory (e.g., `C:\Users\username`)
- `%APPDATA%` - Application data directory (e.g., `C:\Users\username\AppData\Roaming`)
- `%USERNAME%` - Current username
- Any other Windows environment variable

**Examples:**
```yaml
vaults:
  work:
    name: work
    path: "%USERPROFILE%/Documents/work-journal"
    
  appdata:
    name: appdata
    path: "%APPDATA%/journey"
    
  custom:
    name: custom
    path: "%USERPROFILE%/Documents/%USERNAME%_journal"
```

#### Cross-Platform Compatibility
- **Unix/Linux/macOS**: Tilde expansion (`~/path`) works automatically
- **Windows**: Environment variable expansion (`%VAR%`) works automatically
- **Fallback**: If expansion fails, the original path is used as-is

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
- `{date:MM}` / `{{date:MM}}` - Zero-padded month (e.g., 01, 12) - Journals plugin compatibility
- `{date}` - Day number (e.g., 24)
- `{date:02}` - Zero-padded day (e.g., 24, 05)
- `{date:y}` / `{{date:y}}` - Two-digit year (e.g., 25, 24) - Journals plugin compatibility
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

**Journals Plugin Compatibility Examples:**
- `"{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md"` → `25/01/01-15.md`
- `"journals/{date:y}/{date:MM}/{date:02}.md"` → `journals/25/01/15.md`
- `"Personal/{{date:y}}/{{date:MM}}/{{date:MM}}-{day:02}.md"` → `Personal/25/01/01-15.md`

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
# Create a vault (Unix/Linux/macOS)
journeyctl init --path ~/journal --name daily

# Create a vault (Windows)
journeyctl init --path "%USERPROFILE%\journal" --name daily

# Initialize from Obsidian vault
journeyctl init --path ~/Documents/MyObsidianVault --obsidian

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
