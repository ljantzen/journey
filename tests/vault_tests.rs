use journey::vault::Vault;
use journey::config::VaultConfig;
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
    let timestamp = Local::now();
    
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
    let timestamp = Local::now();
    
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
    let timestamp = Local::now();
    
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
    };
    let vault = Vault::new(config);
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let timestamp = Local::now();
    
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
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    
    // Create initial file with existing content
    let initial_content = "# Other Section\n\n- [09:00] Old note\n\n# Daily Notes\n\n- [10:00] First note\n\n# Another Section\n\n- [11:00] Other note";
    let note_path = vault.get_note_path(date);
    std::fs::create_dir_all(note_path.parent().unwrap()).unwrap();
    std::fs::write(&note_path, initial_content).unwrap();
    
    // Add a new note to the Daily Notes section
    vault.add_note("New note in section", None).unwrap();
    
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
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    
    // Create initial file without the target section
    let initial_content = "# Other Section\n\n- [09:00] Old note\n\n# Another Section\n\n- [11:00] Other note";
    let note_path = vault.get_note_path(date);
    std::fs::create_dir_all(note_path.parent().unwrap()).unwrap();
    std::fs::write(&note_path, initial_content).unwrap();
    
    // Add a new note - should create the Daily Notes section
    vault.add_note("New note in new section", None).unwrap();
    
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
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    
    // Add note to new file - should create section
    vault.add_note("First note in new file", None).unwrap();
    
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
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let timestamp = Local::now();
    
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
    };
    let vault = Vault::new(config);
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let timestamp = Local::now();
    
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
    };
    let vault = Vault::new(config);
    
    let _date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let timestamp = Local::now();
    
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
    
    // Test phrase expansion
    vault_with_phrases.add_note("@meeting went well", None).unwrap();
    vault_with_phrases.add_note("@lunch and then @work", None).unwrap();
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
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
    
    // Test that longer phrase is matched first
    vault_with_phrases.add_note("Did @workout today", None).unwrap();
    
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let note_path = vault_with_phrases.get_note_path(date);
    let content = std::fs::read_to_string(&note_path).unwrap();
    
    // Verify longer phrase was matched, not the shorter one
    assert!(content.contains("Gym session completed"));
    assert!(!content.contains("Working"));
}
