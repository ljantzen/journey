use journey::vault::Vault;
use journey::config::VaultConfig;
use std::collections::HashMap;
use tempfile::TempDir;
use chrono::{Local, NaiveDate};

fn create_test_vault() -> (Vault, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
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
