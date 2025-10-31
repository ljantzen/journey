# Journey - A CLI-based Journal Application

Journey is a command-line journal application written in Rust that allows you to manage daily notes with automatic timestamping and flexible date/time handling.

## Features

- **Vault Management**: Create and manage multiple journal vaults
- **Obsidian Integration**: Import configuration from existing Obsidian vaults and plugins
- **Automatic Timestamping**: Notes are automatically timestamped
- **Flexible Date/Time**: Support for absolute dates, relative dates, and custom times
- **Category-Based Sections**: Organize notes into specific sections (work, personal, health, meetings)
- **Template Files**: Custom templates with variable substitution
- **Phrase Expansion**: Define shortcuts that expand to full phrases
- **Table & Bullet Formats**: Choose between bullet points or table format for notes
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Dual Binary Architecture**: Separate `journey` and `journeyctl` commands

## Installation

### Using Cargo

```bash
cargo install journey-cli
```

### Build from Source

```bash
git clone https://github.com/ljantzen/journey.git
cd journey
cargo build --release
cargo install --path .
```

### Download Binaries

Download directly from the [GitHub releases page](https://github.com/ljantzen/journey/releases).

## Quick Start

### 1. Initialize a Vault

```bash
# Unix/Linux/macOS
journeyctl init --path ~/my-journal --name personal

# Windows
journeyctl init --path "%USERPROFILE%\my-journal" --name personal

# From existing Obsidian vault
journeyctl init --path ~/Documents/MyObsidianVault --obsidian
```

### 2. Add Notes

```bash
# Add a note for today
journey This is my first note

# Add a note for yesterday
journey --relative-date 1 Note for yesterday

# Add a note with specific time
journey --time 14:30 Afternoon meeting notes
```

### 3. List Notes

```bash
# List today's notes (default behavior)
journey

# List notes for specific date
journey --date 2025-10-24 --list

# List with table headers (if using table format)
journey --list --header
```

### 4. Organize with Categories

```bash
# Add notes to specific sections
journey -c work "Completed quarterly report"
journey -c personal "Had dinner with friends"
journey -c health "30 minutes of cardio"
```

## Binary Architecture

Journey uses two separate binaries:

### `journey` - Note Operations
Main journal application for daily note operations:
- Add notes
- List notes
- Edit notes

### `journeyctl` - Vault Management
Administrative tool for vault configuration:
- Initialize vaults
- Manage default vault
- List/unlist vaults
- Show today's file path

## Core Concepts

### Vaults
A vault is a directory containing your journal notes. You can have multiple vaults for different purposes (work, personal, projects, etc.).

### Notes & Timestamps
Each note is automatically timestamped when created. Notes are stored in daily markdown files with frontmatter.

### Categories
Organize notes within daily files using categories. Each category can have its own section header (e.g., "Work Notes", "Personal Notes").

## Basic Commands

### Adding Notes

```bash
# Basic note
journey My note content

# With date
journey --date 2025-10-24 Note for specific date

# Relative dates (positive = past, negative = future)
journey --relative-date 1 Note for yesterday
journey --relative-date -1 Note for tomorrow

# With time
journey --time 14:30 Note with specific time

# With category
journey -c work "Work-related note"

# From stdin (each line becomes a note)
echo -e "First note\nSecond note" | journey --stdin
```

### Listing Notes

```bash
# List today's notes
journey
journey --list

# List for specific date
journey --date 2025-10-24 --list

# List by category
journey --list -c work

# With table headers (table format only)
journey --list --header
```

### Editing Notes

```bash
# Edit today's notes
journey --edit

# Edit specific date
journey --date 2025-10-24 --edit
```

## Vault Management

### Initialize Vaults

```bash
# Regular vault
journeyctl init --path ~/journal --name personal

# From Obsidian vault (auto-detects plugins)
journeyctl init --path ~/Documents/ObsidianVault --obsidian
```

### Default Vault

```bash
# Set default vault
journeyctl set-default personal

# Show current default
journeyctl show-default

# Unset default
journeyctl unset-default
```

### List and Manage Vaults

```bash
# List all vaults
journeyctl list

# Unlist a vault (removes from config, doesn't delete files)
journeyctl unlist-vault vault-name

# Show today's file path
journeyctl today
journeyctl today --vault vault-name
journeyctl today --verbose
```

## Configuration

Configuration file location:
- **Linux**: `~/.config/journey/journey.yaml`
- **macOS**: `~/Library/Application Support/journey/journey.yaml`
- **Windows**: `%APPDATA%\journey\journey.yaml`

### Basic Configuration

```yaml
vaults:
  personal:
    name: personal
    path: ~/Documents/journal
    locale: en_US.UTF-8
    list_type: bullet  # or "table"
    section_header: "Daily Log"
    phrases: {}
```

### Advanced Configuration

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
      "@meeting": "Team meeting about project status"
      "@standup": "Daily standup meeting"
      "@review": "Code review completed"
```

## Advanced Features

### Categories and Sections
Organize notes into different sections within your daily files. See [docs/CATEGORIES.md](docs/CATEGORIES.md) for details.

```bash
journey -c work "Completed deployment"
journey -c personal "Dinner with family"
```

### Template Files
Use custom templates for new daily files with variable substitution. See [docs/TEMPLATE_VARIABLES.md](docs/TEMPLATE_VARIABLES.md) for details.

```yaml
template_file: ~/templates/daily.md
```

### Phrase Expansion
Define shortcuts that expand to full phrases. See [docs/PHRASE_EXPANSION.md](docs/PHRASE_EXPANSION.md) for details.

```yaml
phrases:
  "@meeting": "Team meeting about project status"
```

### Custom File Paths
Organize notes in any directory structure. See [docs/FILE_PATH_FORMATS.md](docs/FILE_PATH_FORMATS.md) for details.

```yaml
file_path_format: "{year}/{month:02}/{date:02}.md"
```

### Table Mode
Display notes in table format with customizable headers. See [docs/TABLE_MODE.md](docs/TABLE_MODE.md) for details.

```yaml
list_type: table
```

### Obsidian Integration
Import configuration from Obsidian vaults and plugins. See [docs/OBSIDIAN_INTEGRATION.md](docs/OBSIDIAN_INTEGRATION.md) for details.

```bash
journeyctl init --path ~/ObsidianVault --obsidian
```

### Localization
Support for multiple locales with locale-aware date parsing and table headers. See [docs/LOCALIZATION.md](docs/LOCALIZATION.md) for details.

```yaml
locale: no_NO.UTF-8  # Norwegian
```

## Command Reference

### journey (Note Operations)

| Option | Description |
|--------|-------------|
| `<NOTE>` | Add a note (default action) |
| `-l, --list` | List notes for the specified date |
| `-e, --edit` | Edit notes for the specified date |
| `-d, --date <DATE>` | Specify date (multiple formats supported) |
| `-r, --relative-date <DAYS>` | Days offset (positive=past, negative=future) |
| `-t, --time <TIME>` | Specify time (HH:MM or HH:MM:SS) |
| `--time-format <FORMAT>` | Force time format (12h or 24h) |
| `-c, --category <CATEGORY>` | Specify category (work, personal, health, meetings) |
| `--header` | Include table headers when listing (table format only) |
| `--stdin` | Read input from stdin (each line = one note) |
| `-V, --vault <NAME>` | Specify vault name |
| `-v, --version` | Show version information |

### journeyctl (Vault Management)

| Command | Description |
|---------|-------------|
| `init --path <PATH>` | Initialize a new vault |
| `init --path <PATH> --obsidian` | Initialize from Obsidian vault |
| `list` | List all configured vaults |
| `set-default <NAME>` | Set default vault |
| `show-default` | Show current default vault |
| `unset-default` | Remove default vault |
| `unlist-vault <NAME>` | Remove vault from configuration |
| `today` | Show today's file path |
| `today --vault <NAME>` | Show path for specific vault |
| `today --verbose` | Show detailed information |

## Date Format Support

Journey supports multiple date formats based on your locale:

**English/US (`en_US.UTF-8`):**
- `2025-10-24` (ISO)
- `10/24/2025` (US)
- `October 24, 2025` (Long)

**Norwegian (`no_NO.UTF-8`):**
- `2025-10-24` (ISO)
- `24.10.2025` (Norwegian)
- `24. oktober 2025` (Long)

**Custom override:**
```yaml
date_format: "DD.MM.YYYY"
```

## Path Expansion

Journey supports automatic path expansion:

**Unix/Linux/macOS:**
```yaml
path: ~/Documents/journal  # Expands to /home/user/Documents/journal
```

**Windows:**
```yaml
path: "%USERPROFILE%\Documents\journal"  # Expands to C:\Users\user\Documents\journal
```

## File Structure

Default structure (can be customized):

```
vault-directory/
├── 2025-10-24.md
├── 2025-10-23.md
└── 2025-10-22.md
```

Each markdown file contains:

```markdown
---
date: 2025-10-24
---

- 13:35:27 This is my first note
- 13:35:35 This is my second note
```

Or in table format:

```markdown
---
date: 2025-10-24
---

| 13:35:27 | This is my first note |
| 13:35:35 | This is my second note |
```

## Examples

### Daily Workflow

```bash
# Morning: Add standup notes
journey -c work "@standup discussed project status"

# Afternoon: Log activities
journey -c work "Completed code review"
journey -c personal "Lunch with colleague"

# Evening: Review the day
journey --list
```

### Working with Multiple Vaults

```bash
# Set default vault
journeyctl set-default work

# Add to default vault
journey "Note in work vault"

# Add to specific vault
journey --vault personal "Note in personal vault"
```

### Batch Operations

```bash
# Import notes from file
cat notes.txt | journey --stdin

# Add multiple categorized notes
echo -e "Task 1\nTask 2\nTask 3" | journey --stdin -c work
```

### Integration with Other Tools

```bash
# Export today's notes
journey --list --header > daily-report.md

# Copy to clipboard (Linux)
journey --list --header | xclip -selection clipboard

# Email notes
journey --list --header | mail -s "Daily Notes" team@example.com

# Find today's file for external editing
vim $(journeyctl today)
```

## Documentation

Detailed documentation is available in the [docs/](docs/) directory:

- **[OBSIDIAN_INTEGRATION.md](docs/OBSIDIAN_INTEGRATION.md)** - Obsidian vault integration
- **[CATEGORIES.md](docs/CATEGORIES.md)** - Category-based section organization
- **[TEMPLATE_VARIABLES.md](docs/TEMPLATE_VARIABLES.md)** - Template file variables
- **[PHRASE_EXPANSION.md](docs/PHRASE_EXPANSION.md)** - Phrase shortcuts
- **[FILE_PATH_FORMATS.md](docs/FILE_PATH_FORMATS.md)** - Custom file path formats
- **[TABLE_MODE.md](docs/TABLE_MODE.md)** - Table format and headers
- **[LOCALIZATION.md](docs/LOCALIZATION.md)** - Locale support and internationalization
- **[SECTION_BASED_NOTES.md](docs/SECTION_BASED_NOTES.md)** - Section-based note organization
- **[DATE_FORMAT_OVERRIDE.md](docs/DATE_FORMAT_OVERRIDE.md)** - Custom date formats
- **[TIME_FORMAT_OVERRIDE.md](docs/TIME_FORMAT_OVERRIDE.md)** - Time format overrides
- **[DEFAULT_BEHAVIOR_UPDATE.md](docs/DEFAULT_BEHAVIOR_UPDATE.md)** - Default behavior changes
- **[SINGLE_VAULT_AUTO_SELECTION.md](docs/SINGLE_VAULT_AUTO_SELECTION.md)** - Single vault auto-selection

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- --help
```

## License

This software is licensed under a combined MIT and SPPL license. It is basically a MIT license, but in order to be compliant you need to send me a postcard. Details in [LICENSE](https://github.com/ljantzen/journey/blob/main/LICENSE).
