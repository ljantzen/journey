# Localization and Internationalization

Journey provides comprehensive locale support for date/time parsing, formatting, and table headers.

## Locale Configuration

Set the locale in your vault configuration:

```yaml
vaults:
  personal:
    name: personal
    path: ~/journal
    locale: en_US.UTF-8  # US English
```

## Supported Locales

Journey supports all standard locales. Common examples:

### English Variants
- `en_US.UTF-8` - US English
- `en_GB.UTF-8` - British English
- `en_CA.UTF-8` - Canadian English
- `en_AU.UTF-8` - Australian English

### European Languages
- `no_NO.UTF-8` - Norwegian
- `sv_SE.UTF-8` - Swedish
- `da_DK.UTF-8` - Danish
- `fi_FI.UTF-8` - Finnish
- `de_DE.UTF-8` - German
- `fr_FR.UTF-8` - French
- `es_ES.UTF-8` - Spanish
- `it_IT.UTF-8` - Italian
- `nl_NL.UTF-8` - Dutch
- `pt_PT.UTF-8` - Portuguese

### Asian Languages
- `ja_JP.UTF-8` - Japanese
- `zh_CN.UTF-8` - Chinese (Simplified)
- `zh_TW.UTF-8` - Chinese (Traditional)
- `ko_KR.UTF-8` - Korean

### Other Languages
- `ru_RU.UTF-8` - Russian
- `pl_PL.UTF-8` - Polish
- `tr_TR.UTF-8` - Turkish
- `ar_SA.UTF-8` - Arabic

## Date Format Support by Locale

### English/US Locale (`en_US.UTF-8`)

Supported formats:
- `2025-10-24` (ISO format)
- `10/24/2025` (US format)
- `10-24-2025` (US with dashes)
- `October 24, 2025` (Long format)
- `Oct 24, 2025` (Short format)

Example:
```bash
journey --date 10/24/2025 "US format date"
journey --date "October 24, 2025" "Long format"
```

### Norwegian Locale (`no_NO.UTF-8`)

Supported formats:
- `2025-10-24` (ISO format)
- `24.10.2025` (Norwegian format)
- `24/10/2025` (European format)
- `24-10-2025` (European with dashes)
- `24. oktober 2025` (Norwegian long)
- `24. okt 2025` (Norwegian short)

Example:
```bash
journey --date 24.10.2025 "Norwegian format"
journey --date "24. oktober 2025" "Norwegian long"
```

### German Locale (`de_DE.UTF-8`)

Supported formats:
- `2025-10-24` (ISO format)
- `24.10.2025` (German format)
- `24/10/2025` (European format)
- `24. Oktober 2025` (German long)
- `24. Okt 2025` (German short)

### French Locale (`fr_FR.UTF-8`)

Supported formats:
- `2025-10-24` (ISO format)
- `24/10/2025` (French format)
- `24 octobre 2025` (French long)
- `24 oct 2025` (French short)

## Table Headers by Locale

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

### Example Output

**English:**
```markdown
| Time | Content |
|------|----------|
| 09:00 | Morning standup meeting |
```

**Norwegian:**
```markdown
| Tid | Innhold |
|-----|----------|
| 09:00 | Morgen standup møte |
```

**German:**
```markdown
| Zeit | Inhalt |
|------|--------|
| 09:00 | Morgen Standup Meeting |
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

Output:
```markdown
| Timestamp | Note |
|-----------|------|
| 09:00 | Morning standup meeting |
```

## Time Format Support

### 12-Hour Format (US/UK)
```bash
# Automatic detection for US locale
journey --time 2:30PM "Afternoon meeting"
journey --time 2:30:45PM "Precise time"
```

### 24-Hour Format (Most of Europe/Asia)
```bash
# Automatic detection for European locales
journey --time 14:30 "Afternoon meeting"
journey --time 14:30:45 "Precise time"
```

### Force Specific Format
```bash
# Force 12-hour format
journey --time 2:30PM --time-format 12h "Force 12h"

# Force 24-hour format
journey --time 14:30 --time-format 24h "Force 24h"
```

## Weekday and Month Names

Weekday and month names in file paths and templates respect the locale:

### English (`en_US.UTF-8`)
```yaml
file_path_format: "{year}/{Month}/{Weekday}-{date:02}.md"
# Output: 2025/October/Friday-24.md
```

### Norwegian (`no_NO.UTF-8`)
```yaml
file_path_format: "{year}/{Month}/{Weekday}-{date:02}.md"
# Output: 2025/oktober/fredag-24.md
```

### German (`de_DE.UTF-8`)
```yaml
file_path_format: "{year}/{Month}/{Weekday}-{date:02}.md"
# Output: 2025/Oktober/Freitag-24.md
```

### Japanese (`ja_JP.UTF-8`)
```yaml
file_path_format: "{year}/{Month}/{Weekday}-{date:02}.md"
# Output: 2025/10月/金曜日-24.md
```

## Date Format Override

You can override the default date format for a vault:

```yaml
vaults:
  european:
    locale: en_GB.UTF-8
    date_format: "DD.MM.YYYY"  # European format
  
  us:
    locale: en_US.UTF-8
    date_format: "MM/DD/YYYY"  # US format
  
  iso:
    locale: en_US.UTF-8
    date_format: "YYYY-MM-DD"  # ISO format
```

## Best Practices

### Consistent Locale Usage
Use the same locale across all vaults that share notes or templates:

```yaml
vaults:
  work:
    locale: en_US.UTF-8
  personal:
    locale: en_US.UTF-8
```

### Regional Defaults
Choose locales that match your region's conventions:

```yaml
# Norway
vaults:
  personal:
    locale: no_NO.UTF-8
    date_format: "DD.MM.YYYY"

# United States
vaults:
  personal:
    locale: en_US.UTF-8
    date_format: "MM/DD/YYYY"

# Germany
vaults:
  personal:
    locale: de_DE.UTF-8
    date_format: "DD.MM.YYYY"
```

### Multi-Language Environments
Create separate vaults for different languages:

```yaml
vaults:
  english-journal:
    locale: en_US.UTF-8
    path: ~/journal-en
  
  norwegian-journal:
    locale: no_NO.UTF-8
    path: ~/journal-no
```

## Troubleshooting

### Date Not Parsing
- Check that the date format matches your locale
- Try ISO format (`YYYY-MM-DD`) which works for all locales
- Verify locale is correctly set in configuration

### Wrong Table Headers
- Check `locale` setting in vault configuration
- Override with custom `table_headers` if needed
- Ensure locale is installed on your system

### Incorrect Month/Weekday Names
- Verify locale setting
- Check that the locale is available on your system
- Use `locale -a` (Unix/Linux) to list available locales

### Time Format Issues
- Use `--time-format` to force specific format
- Check locale time format conventions
- Verify time string matches expected format

