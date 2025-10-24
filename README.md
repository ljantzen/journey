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
journey "My note content"

# Add note with explicit command
journey add "My note content"

# Add note for specific date
journey --date 2025-10-22 "Note for specific date"

# Add note for yesterday
journey --relative-date 1 "Note for yesterday"

# Add note with specific time
journey --time "14:30" "Note with specific time"
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

## File Structure

Notes are stored as markdown files with the following structure:

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

- [2025-10-24 13:35:27] This is my first note
- [2025-10-24 13:35:35] This is my second note
```

## Options

- `-d, --date <DATE>`: Specify date in YYYY-MM-DD format
- `-r, --relative-date <DAYS>`: Days ago (0 = today, 1 = yesterday)
- `-t, --time <TIME>`: Specify time in HH:MM or HH:MM:SS format
- `-v, --vault <NAME>`: Specify vault name
- `-l, --list`: List notes
- `-e, --edit`: Edit notes
- `-a, --add-note <NOTE>`: Add a note

## Examples

```bash
# Create a vault
journey init --path ~/journal --name daily

# Add notes throughout the day
journey "Morning coffee and planning"
journey "Completed the project milestone"
journey "Evening reflection on the day"

# Add a note for yesterday
journey --relative-date 1 "Forgot to log this yesterday"

# List all notes for today
journey --list

# Edit today's notes
journey --edit

# Add a note with specific time
journey --time "09:30" "Early morning meeting notes"
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
