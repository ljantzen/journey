use journey::vault::Vault;
use journey::config::{VaultConfig, NoteFormat};
use journey::errors::JourneyError;
use std::collections::HashMap;
use tempfile::TempDir;
use chrono::{Local, NaiveDate, TimeZone};

fn create_test_vault() -> (Vault, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    (Vault::new(config), temp_dir)
}

#[test]
fn test_vault_creation() {
    let (vault, _temp_dir) = create_test_vault();
    assert_eq!(vault.config.name, "test");
    assert_eq!(vault.config.locale, "en-US");
}

#[test]
fn test_get_note_path() {
    let (vault, _temp_dir) = create_test_vault();
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let path = vault.get_note_path(date);
    assert!(path.to_string_lossy().contains("2025-10-24.md"));
}

#[test]
fn test_add_note_new_file() {
    let (vault, _temp_dir) = create_test_vault();
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    vault.add_note("Test note", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    assert!(note_path.exists());
    
    let content = std::fs::read_to_string(&note_path).unwrap();
    assert!(content.contains("Test note"));
    assert!(content.contains("---"));
    assert!(content.contains("date: 2025-10-24"));
}

#[test]
fn test_add_note_existing_file() {
    let (vault, _temp_dir) = create_test_vault();
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add first note
    vault.add_note("First note", Some(timestamp)).unwrap();
    
    // Add second note
    vault.add_note("Second note", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    assert!(content.contains("First note"));
    assert!(content.contains("Second note"));
    
    // Count the number of note entries
    let note_count = content.matches("- [").count();
    assert_eq!(note_count, 2);
}

#[test]
fn test_list_notes_empty() {
    let (vault, _temp_dir) = create_test_vault();
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    
    let notes = vault.list_notes(date).unwrap();
    assert!(notes.is_empty());
}

#[test]
fn test_list_notes_with_content() {
    let (vault, _temp_dir) = create_test_vault();
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    vault.add_note("Test note 1", Some(timestamp)).unwrap();
    vault.add_note("Test note 2", Some(timestamp)).unwrap();
    
    let notes = vault.list_notes(date).unwrap();
    assert_eq!(notes.len(), 2);
    assert!(notes[0].contains("Test note 1"));
    assert!(notes[1].contains("Test note 2"));
}

#[test]
fn test_get_editor_path() {
    let (vault, _temp_dir) = create_test_vault();
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let path = vault.get_editor_path(date);
    assert!(path.to_string_lossy().contains("2025-10-24.md"));
}

#[test]
fn test_vault_with_section() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    vault.add_note("Test note with section", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("Test note with section"));
}

#[test]
fn test_find_section() {
    let (vault, _temp_dir) = create_test_vault();
    
    let content = "---\ndate: 2025-10-24\n---\n\n# Daily Notes\n\n- [2025-10-24 10:00:00] Note 1\n";
    let section_start = vault.find_section(content, "Daily Notes");
    assert!(section_start.is_some());
    assert_eq!(section_start.unwrap(), 4); // Line 5 (0-indexed)
    
    let no_section = vault.find_section(content, "Non-existent Section");
    assert!(no_section.is_none());
}

#[test]
fn test_find_section_end() {
    let (vault, _temp_dir) = create_test_vault();
    
    let lines = vec![
        "# Section 1",
        "",
        "- [10:00] Note 1",
        "",
        "# Section 2",
        "",
        "- [11:00] Note 2"
    ];
    
    // Section 1 should end at line 4 (before Section 2)
    let section_end = vault.find_section_end(&lines, 0);
    assert_eq!(section_end, 4);
    
    // Section 2 should end at line 7 (end of file)
    let section_end = vault.find_section_end(&lines, 4);
    assert_eq!(section_end, 7);
}

#[test]
fn test_add_note_to_existing_section() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    
    // Create initial file with existing content
    let initial_content = "# Other Section\n\n- [09:00] Old note\n\n# Daily Notes\n\n- [10:00] First note\n\n# Another Section\n\n- [11:00] Other note";
    let note_path = vault.get_note_path(date);
    std::fs::create_dir_all(note_path.parent().unwrap()).unwrap();
    std::fs::write(&note_path, initial_content).unwrap();
    
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add a new note to the Daily Notes section
    vault.add_note("New note in section", Some(timestamp)).unwrap();
    
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify the note was added to the correct section
    let lines: Vec<&str> = content.lines().collect();
    let daily_notes_start = vault.find_section(&content, "Daily Notes").unwrap();
    let daily_notes_end = vault.find_section_end(&lines, daily_notes_start);
    
    // The new note should be in the Daily Notes section (before Another Section)
    assert!(daily_notes_end < lines.len());
    assert!(content.contains("New note in section"));
    
    // Verify other sections are preserved
    assert!(content.contains("Old note"));
    assert!(content.contains("Other note"));
}

#[test]
fn test_add_note_create_missing_section() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    
    // Create initial file without the target section
    let initial_content = "# Other Section\n\n- [09:00] Old note\n\n# Another Section\n\n- [11:00] Other note";
    let note_path = vault.get_note_path(date);
    std::fs::create_dir_all(note_path.parent().unwrap()).unwrap();
    std::fs::write(&note_path, initial_content).unwrap();
    
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add a new note - should create the Daily Notes section
    vault.add_note("New note in new section", Some(timestamp)).unwrap();
    
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify the section was created
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("New note in new section"));
    
    // Verify other sections are preserved
    assert!(content.contains("Old note"));
    assert!(content.contains("Other note"));
}

#[test]
fn test_add_note_to_section_new_file() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add note to new file - should create section
    vault.add_note("First note in new file", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify the section was created
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("First note in new file"));
    assert!(content.contains("---"));
    assert!(content.contains("date: 2025-10-24"));
}

#[test]
fn test_add_note_with_template() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a template file
    let template_content = r#"---
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
"#;
    
    let template_path = temp_dir.path().join("template.md");
    std::fs::write(&template_path, template_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: Some(template_path.to_str().unwrap().to_string()),
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add note with template
    vault.add_note("Test note with template", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify template variables were replaced
    assert!(content.contains("date: 2025-10-24"));
    assert!(content.contains("time:"));
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("Test note with template"));
    assert!(content.contains("## Morning"));
    assert!(content.contains("## Afternoon"));
    assert!(content.contains("## Evening"));
}

#[test]
fn test_add_note_with_template_note_placeholder() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a template file with {{note}} placeholder
    let template_content = r#"---
date: {{date}}
---

# {{section_name}}

{{note}}

## Tasks
- [ ] Morning routine
- [ ] Work tasks
"#;
    
    let template_path = temp_dir.path().join("template.md");
    std::fs::write(&template_path, template_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: Some(template_path.to_str().unwrap().to_string()),
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add note with template
    vault.add_note("Test note in placeholder", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify template variables were replaced
    assert!(content.contains("date: 2025-10-24"));
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("Test note in placeholder"));
    assert!(content.contains("## Tasks"));
    assert!(content.contains("- [ ] Morning routine"));
}

#[test]
fn test_add_note_with_template_missing_file() {
    let temp_dir = TempDir::new().unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: Some("/nonexistent/template.md".to_string()),
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Should fail when template file doesn't exist
    let result = vault.add_note("Test note", Some(timestamp));
    assert!(result.is_err());
    
    if let Err(JourneyError::Io(io_error)) = result {
        let error_msg = format!("{}", io_error);
        assert!(error_msg.contains("Failed to read template file"));
        assert!(error_msg.contains("/nonexistent/template.md"));
    } else {
        panic!("Expected Io error");
    }
}

#[test]
fn test_add_note_with_template_different_date() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a template file
    let template_content = r#"---
date: {{date}}
time: {{time}}
datetime: {{datetime}}
---

# {{section_name}}

## Notes
{{note}}
"#;
    
    let template_path = temp_dir.path().join("template.md");
    std::fs::write(&template_path, template_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: Some(template_path.to_str().unwrap().to_string()),
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    // Add note to a specific date (not today)
    let specific_date = NaiveDate::from_ymd_opt(2025, 10, 20).unwrap();
    let specific_time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let specific_timestamp = chrono::Local.from_local_datetime(&chrono::NaiveDateTime::new(specific_date, specific_time)).single().unwrap();
    
    vault.add_note("Test note for specific date", Some(specific_timestamp)).unwrap();
    
    let note_path = vault.get_note_path(specific_date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify template variables reflect the note's date/time, not current date/time
    assert!(content.contains("date: 2025-10-20"));
    assert!(content.contains("time: 14:30:00"));
    assert!(content.contains("datetime:"));
    assert!(content.contains("2025-10-20"));
    assert!(content.contains("14:30:00"));
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("Test note for specific date"));
}

#[test]
fn test_phrase_expansion() {
    let (vault, _temp_dir) = create_test_vault();
    
    // Add some phrases to the vault config
    let mut config = vault.config.clone();
    config.phrases.insert("@meeting".to_string(), "Meeting with team about project status".to_string());
    config.phrases.insert("@lunch".to_string(), "Had lunch at the usual place".to_string());
    config.phrases.insert("@work".to_string(), "Working on important tasks".to_string());
    
    let vault_with_phrases = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Test phrase expansion
    vault_with_phrases.add_note("@meeting went well", Some(timestamp)).unwrap();
    vault_with_phrases.add_note("@lunch and then @work", Some(timestamp)).unwrap();
    let note_path = vault_with_phrases.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify phrases were expanded
    assert!(content.contains("Meeting with team about project status"));
    assert!(content.contains("Had lunch at the usual place"));
    assert!(content.contains("Working on important tasks"));
    
    // Verify original phrases are not in the content
    assert!(!content.contains("@meeting"));
    assert!(!content.contains("@lunch"));
    assert!(!content.contains("@work"));
}

#[test]
fn test_phrase_expansion_longest_first() {
    let (vault, _temp_dir) = create_test_vault();
    
    // Add phrases where one is a substring of another
    let mut config = vault.config.clone();
    config.phrases.insert("@work".to_string(), "Working".to_string());
    config.phrases.insert("@workout".to_string(), "Gym session completed".to_string());
    
    let vault_with_phrases = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Test that longer phrase is matched first
    vault_with_phrases.add_note("Did @workout today", Some(timestamp)).unwrap();
    let note_path = vault_with_phrases.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify longer phrase was matched, not the shorter one
    assert!(content.contains("Gym session completed"));
    assert!(!content.contains("Working"));
}

#[test]
fn test_custom_file_path_format() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: Some("work/{year}/{month}/{date}.md".to_string()),
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let note_path = vault.get_note_path(date);
    
    // Verify the path uses the custom format
    assert!(note_path.to_string_lossy().contains("work/2025/10/24.md"));
}

#[test]
fn test_custom_file_path_format_with_day() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: Some("journals/{year}/{month:02}/{day:02}.md".to_string()),
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 3, 5).unwrap();
    let note_path = vault.get_note_path(date);
    
    // Verify the path uses the custom format with zero-padded values
    assert!(note_path.to_string_lossy().contains("journals/2025/03/05.md"));
}

#[test]
fn test_custom_file_path_format_with_weekday() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: Some("daily/{Weekday}/{year}-{month:02}-{date:02}.md".to_string()),
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    // Test with a known weekday (2025-10-24 is a Friday)
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let note_path = vault.get_note_path(date);
    
    // Verify the path uses the custom format with weekday
    assert!(note_path.to_string_lossy().contains("daily/Friday/2025-10-24.md"));
}

#[test]
fn test_custom_file_path_format_with_short_weekday() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: Some("notes/{Weekday_short}_{year}-{month:02}-{date:02}.md".to_string()),
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    // Test with a known weekday (2025-10-24 is a Friday)
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let note_path = vault.get_note_path(date);
    
    // Verify the path uses the custom format with short weekday
    assert!(note_path.to_string_lossy().contains("notes/Fri_2025-10-24.md"));
}

#[test]
fn test_custom_file_path_format_case_sensitive_weekday() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: Some("test_{Weekday}_{weekday}_{Weekday_short}_{weekday_short}.md".to_string()),
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    // Test with a known weekday (2025-10-24 is a Friday)
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let note_path = vault.get_note_path(date);
    
    // Verify the path uses case-sensitive weekday formats
    assert!(note_path.to_string_lossy().contains("test_Friday_friday_Fri_fri.md"));
}

#[test]
fn test_custom_file_path_format_case_sensitive_month() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: Some("test_{Month}_{month_name}_{Month_short}_{month_short}.md".to_string()),
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: None,
    };
    let vault = Vault::new(config);
    
    // Test with October (month 10)
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let note_path = vault.get_note_path(date);
    
    // Verify the path uses case-sensitive month formats
    assert!(note_path.to_string_lossy().contains("test_October_october_Oct_oct.md"));
}

#[test]
fn test_note_format_bullet_default() {
    let (vault, _temp_dir) = create_test_vault();
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    vault.add_note("Test bullet note", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Should use bullet format by default
    assert!(content.contains("- ["));
    assert!(content.contains("Test bullet note"));
}

#[test]
fn test_note_format_table() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: Some(NoteFormat::Table),
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    vault.add_note("Test table note", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Should use table format
    assert!(content.contains("| Time | Content |"));
    assert!(content.contains("|------|----------|"));
    assert!(content.contains("|"));
    assert!(content.contains("Test table note"));
}

#[test]
fn test_note_format_conversion_bullet_to_table() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create initial file with bullet format
    let initial_content = "---\ndate: 2025-10-24\n---\n\n- [10:00:00] First note\n- [11:00:00] Second note\n";
    let note_path = temp_dir.path().join("2025-10-24.md");
    std::fs::write(&note_path, initial_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: Some(NoteFormat::Table),
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add a new note - should convert existing bullet notes to table format
    vault.add_note("Third note", Some(timestamp)).unwrap();
    
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Should have table format
    assert!(content.contains("| Time | Content |"));
    assert!(content.contains("|------|----------|"));
    assert!(content.contains("| 10:00:00 | First note |"));
    assert!(content.contains("| 11:00:00 | Second note |"));
    assert!(content.contains("|"));
    assert!(content.contains("Third note"));
}

#[test]
fn test_note_format_conversion_table_to_bullet() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create initial file with table format
    let initial_content = "---\ndate: 2025-10-24\n---\n\n| Time | Content |\n|------|----------|\n| 10:00:00 | First note |\n| 11:00:00 | Second note |\n";
    let note_path = temp_dir.path().join("2025-10-24.md");
    std::fs::write(&note_path, initial_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: Some(NoteFormat::Bullet),
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add a new note - should convert existing table notes to bullet format
    vault.add_note("Third note", Some(timestamp)).unwrap();
    
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Should have bullet format
    assert!(content.contains("- [10:00:00] First note"));
    assert!(content.contains("- [11:00:00] Second note"));
    assert!(content.contains("- ["));
    assert!(content.contains("Third note"));
}

#[test]
fn test_note_format_detection() {
    let (vault, _temp_dir) = create_test_vault();
    
    // Test bullet format detection
    let bullet_content = "- [10:00:00] Test note\n- [11:00:00] Another note\n";
    let detected_format = vault.detect_note_format(bullet_content);
    assert_eq!(detected_format, Some(NoteFormat::Bullet));
    
    // Test table format detection
    let table_content = "| Time | Content |\n|------|----------|\n| 10:00:00 | Test note |\n| 11:00:00 | Another note |\n";
    let detected_format = vault.detect_note_format(table_content);
    assert_eq!(detected_format, Some(NoteFormat::Table));
    
    // Test mixed format (should return None)
    let mixed_content = "- [10:00:00] Test note\n| 11:00:00 | Another note |\n";
    let detected_format = vault.detect_note_format(mixed_content);
    assert_eq!(detected_format, None);
    
    // Test no notes (should return None)
    let no_notes_content = "---\ndate: 2025-10-24\n---\n\n# Daily Notes\n\nSome other content\n";
    let detected_format = vault.detect_note_format(no_notes_content);
    assert_eq!(detected_format, None);
}

#[test]
fn test_list_notes_table_format() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: Some(NoteFormat::Table),
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    vault.add_note("First table note", Some(timestamp)).unwrap();
    vault.add_note("Second table note", Some(timestamp)).unwrap();
    
    let notes = vault.list_notes(date).unwrap();
    assert_eq!(notes.len(), 2);
    
    // Notes should be in table format
    assert!(notes[0].contains("|"));
    assert!(notes[0].contains("First table note"));
    assert!(notes[1].contains("|"));
    assert!(notes[1].contains("Second table note"));
}

#[test]
fn test_note_format_with_section() {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: Some(NoteFormat::Table),
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    vault.add_note("Test note with section", Some(timestamp)).unwrap();
    
    let note_path = vault.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Should have section and table format
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("| Time | Content |"));
    assert!(content.contains("|------|----------|"));
    assert!(content.contains("|"));
    assert!(content.contains("Test note with section"));
}

#[test]
fn test_note_format_conversion_with_section() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create initial file with bullet format in a section
    let initial_content = "---\ndate: 2025-10-24\n---\n\n# Daily Notes\n\n- [10:00:00] First note\n- [11:00:00] Second note\n\n# Other Section\n\n- [12:00:00] Other note\n";
    let note_path = temp_dir.path().join("2025-10-24.md");
    std::fs::write(&note_path, initial_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: Some("Daily Notes".to_string()),
        date_format: None,
        template_file: None,
        file_path_format: None,
        weekly_format: None,
        monthly_format: None,
        quarterly_format: None,
        yearly_format: None,
        note_format: Some(NoteFormat::Table),
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp = Local.from_local_datetime(&chrono::NaiveDateTime::new(date, time)).single().unwrap();
    
    // Add a new note - should convert only the Daily Notes section
    vault.add_note("Third note", Some(timestamp)).unwrap();
    
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // All notes should be converted to table format (global setting)
    assert!(content.contains("# Daily Notes"));
    assert!(content.contains("| 10:00:00 | First note |"));
    assert!(content.contains("| 11:00:00 | Second note |"));
    assert!(content.contains("|"));
    assert!(content.contains("Third note"));
    
    // Other section should also be converted to table format
    assert!(content.contains("# Other Section"));
    assert!(content.contains("| 12:00:00 | Other note |"));
}
