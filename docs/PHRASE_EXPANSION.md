# Phrase Expansion

Journey supports custom phrase expansion to make note-taking faster and more consistent.

## Configuration

Define phrases in your vault configuration:

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
      "@standup": "Daily standup meeting"
      "@deploy": "Deployment to production"
    section_header: "Daily Log"
```

## Usage

When you add a note containing a phrase key, it gets automatically replaced with the corresponding value:

```bash
# This note:
journey "@meeting went well, then @lunch"

# Becomes this in your journal:
# - 14:30:00 Team meeting about project status went well, then Had lunch at the usual place
```

## Phrase Features

### Longest Match First
If you have overlapping phrases, Journey matches the longest one first:

```yaml
phrases:
  "@work": "Work session"
  "@workout": "Exercise session at the gym"
```

```bash
# This will match "@workout", not "@work"
journey "@workout completed"
# Output: - 14:30:00 Exercise session at the gym completed
```

### Multiple Phrases
You can use multiple phrases in a single note:

```bash
journey "@standup then @code then @review"
# Output: - 14:30:00 Daily standup meeting then Coding session on main project then Code review completed
```

### Case Sensitive
Phrases are matched exactly as defined:

```yaml
phrases:
  "@Meeting": "Formal meeting"
  "@meeting": "Informal meeting"
```

```bash
journey "@Meeting with executives"
# Output: - 14:30:00 Formal meeting with executives

journey "@meeting with team"
# Output: - 14:30:00 Informal meeting with team
```

### Global Replacement
All occurrences of a phrase in a note are replaced:

```bash
journey "@code and @code and @code"
# Output: - 14:30:00 Coding session on main project and Coding session on main project and Coding session on main project
```

## Common Use Cases

### Daily Activities

```yaml
phrases:
  "@standup": "Daily standup meeting"
  "@lunch": "Lunch break"
  "@eod": "End of day wrap-up"
  "@planning": "Sprint planning session"
  "@retro": "Sprint retrospective"
```

### Project Shortcuts

```yaml
phrases:
  "@proj-a": "Project Alpha development"
  "@proj-b": "Project Beta testing"
  "@proj-c": "Project Charlie documentation"
```

### Meeting Types

```yaml
phrases:
  "@1on1": "One-on-one meeting with manager"
  "@team": "Team sync meeting"
  "@client": "Client presentation"
  "@demo": "Product demo session"
```

### Status Updates

```yaml
phrases:
  "@started": "Started working on"
  "@completed": "Completed task:"
  "@blocked": "Blocked by:"
  "@review": "Submitted for review:"
```

### Personal Tracking

```yaml
phrases:
  "@exercise": "30 minutes of exercise"
  "@reading": "Reading session"
  "@meditation": "Meditation practice"
  "@water": "Drank water"
```

## Advanced Patterns

### Contextual Phrases

Create phrases that work well in different contexts:

```yaml
phrases:
  "@mtg": "meeting"
  "@dev": "development work"
  "@bug": "bug fix"
  "@feat": "feature implementation"
```

```bash
journey "Morning @mtg about @feat"
# Output: - 09:00:00 Morning meeting about feature implementation

journey "Afternoon @dev for @bug"
# Output: - 14:00:00 Afternoon development work for bug fix
```

### Time-Saving Abbreviations

```yaml
phrases:
  "@wfh": "Working from home today"
  "@ooo": "Out of office"
  "@pto": "Paid time off"
  "@sick": "Sick day"
```

### Emoji Shortcuts

```yaml
phrases:
  "@check": "✅"
  "@fire": "🔥"
  "@rocket": "🚀"
  "@bug": "🐛"
  "@idea": "💡"
```

```bash
journey "@check Completed deployment @rocket"
# Output: - 14:30:00 ✅ Completed deployment 🚀
```

## Best Practices

### Use Consistent Prefixes
Start all phrases with a consistent character (e.g., `@`, `#`, `!`):

```yaml
phrases:
  "@meeting": "Team meeting"
  "@code": "Coding session"
  "@review": "Code review"
```

### Keep Phrases Short
Use short, memorable keys:

```yaml
# Good
phrases:
  "@mtg": "meeting"
  "@dev": "development"

# Less ideal (too long)
phrases:
  "@team-meeting-about-project": "Team meeting about project"
```

### Group Related Phrases
Organize phrases by category in your configuration:

```yaml
phrases:
  # Meetings
  "@standup": "Daily standup"
  "@1on1": "One-on-one meeting"
  
  # Development
  "@code": "Coding session"
  "@review": "Code review"
  "@deploy": "Deployment"
  
  # Personal
  "@lunch": "Lunch break"
  "@exercise": "Exercise session"
```

### Document Your Phrases
Keep a reference list of your phrases:

```bash
# Show all configured phrases
cat ~/.config/journey/journey.yaml | grep -A 20 "phrases:"
```

### Avoid Conflicts
Don't create phrases that are substrings of common words:

```yaml
# Problematic - "at" is too common
phrases:
  "at": "attended meeting"

# Better
phrases:
  "@at": "attended meeting"
```

## Combining with Other Features

### Phrases with Categories

```bash
# Add work note with phrase expansion
journey -c work "@standup discussed @proj-a"
# Output in "Work Notes" section: - 09:00:00 Daily standup discussed Project Alpha development
```

### Phrases with Dates

```bash
# Add note for yesterday with phrase
journey --relative-date 1 "@forgot to log @meeting"
# Output: - 14:30:00 Forgot to log Team meeting about project status
```

### Phrases with Stdin

```bash
# Expand phrases from file
cat << EOF | journey --stdin
@standup
@code
@review
@eod
EOF
```

## Troubleshooting

### Phrase Not Expanding
- **Check spelling**: Phrase keys are case-sensitive
- **Check configuration**: Verify phrase is defined in vault config
- **Check vault**: Ensure you're using the correct vault with `--vault`

### Unexpected Replacements
- **Check for overlaps**: Longer phrases are matched first
- **Check case**: `@Meeting` and `@meeting` are different
- **Escape if needed**: If you need the literal text, avoid using the phrase prefix

### Performance with Many Phrases
- Journey processes phrases efficiently, even with hundreds defined
- Longest match first ensures correct expansion
- No performance impact on normal note-taking operations

## Migration from Other Systems

### From Obsidian Templates
Convert Obsidian template variables to Journey phrases:

```yaml
# Obsidian: {{meeting}}
# Journey:
phrases:
  "@meeting": "Team meeting"
```

### From Text Expanders
Convert text expander shortcuts to Journey phrases:

```yaml
# TextExpander: ;mtg
# Journey:
phrases:
  "@mtg": "meeting"
```

### From Aliases
Convert shell aliases to Journey phrases:

```yaml
# Bash alias: alias standup="echo 'Daily standup'"
# Journey:
phrases:
  "@standup": "Daily standup"
```

