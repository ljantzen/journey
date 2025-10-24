use journey::vault::Vault;
use journey::config::VaultConfig;
use std::collections::HashMap;
use tempfile::TempDir;
use chrono::{Local, NaiveDate, TimeZone};
use std::fs;

#[test]
fn test_template_variables() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.md");
    
    // Create a test template with the variables you mentioned
    let template_content = r#"---
created: {created}
updated: {created}
---

[[{yesterday}]] [[{tomorrow}]]

## 📅️ {today} {weekday}

## 🎯

## 🕗

## 🔨

## 👀️

{note}"#;
    
    fs::write(&template_path, template_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: Some(template_path.to_string_lossy().to_string()),
        file_path_format: None,
    };
    
    let vault = Vault::new(config);
    
    // Test with a specific date to make results predictable
    let test_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(); // Wednesday
    let timestamp = Local.from_local_date(&test_date).unwrap().and_hms_opt(14, 30, 0).unwrap();
    
    // Create a test note
    let result = vault.add_note("Test note content", Some(timestamp));
    assert!(result.is_ok());
    
    // Read the generated file and check that variables were expanded
    let note_path = vault.get_note_path(test_date);
    let content = fs::read_to_string(&note_path).unwrap();
    
    // Check that template variables were replaced
    assert!(content.contains("2025-01-15 14:30:00")); // {created}
    assert!(content.contains("2025-01-14")); // {yesterday}
    assert!(content.contains("2025-01-16")); // {tomorrow}
    assert!(content.contains("2025-01-15")); // {today}
    assert!(content.contains("Wednesday")); // {weekday}
    assert!(content.contains("Test note content")); // {note}
    
    // Check that the original template variables are not present
    assert!(!content.contains("{created}"));
    assert!(!content.contains("{yesterday}"));
    assert!(!content.contains("{tomorrow}"));
    assert!(!content.contains("{today}"));
    assert!(!content.contains("{weekday}"));
    assert!(!content.contains("{note}"));
}

#[test]
fn test_weekday_variations() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.md");
    
    // Test both {weekday} and {Weekday}
    let template_content = r#"Today is {weekday} ({Weekday})"#;
    fs::write(&template_path, template_content).unwrap();
    
    let config = VaultConfig {
        name: "test".to_string(),
        path: temp_dir.path().to_path_buf(),
        locale: "en-US".to_string(),
        phrases: HashMap::new(),
        section_name: None,
        date_format: None,
        template_file: Some(template_path.to_string_lossy().to_string()),
        file_path_format: None,
    };
    
    let vault = Vault::new(config);
    
    // Test with a specific date (Wednesday)
    let test_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
    let timestamp = Local.from_local_date(&test_date).unwrap().and_hms_opt(12, 0, 0).unwrap();
    
    let result = vault.add_note("", Some(timestamp));
    assert!(result.is_ok());
    
    let note_path = vault.get_note_path(test_date);
    let content = fs::read_to_string(&note_path).unwrap();
    
    // Check that both weekday formats are expanded
    assert!(content.contains("Wednesday")); // {weekday}
    assert!(content.contains("Wed")); // {Weekday}
}
