use journey::errors::JourneyError;
use std::io;

#[test]
fn test_journey_error_display() {
    let error = JourneyError::Config("Test config error".to_string());
    assert!(error.to_string().contains("Configuration error"));
    assert!(error.to_string().contains("Test config error"));
}

#[test]
fn test_journey_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let journey_error: JourneyError = io_error.into();
    assert!(matches!(journey_error, JourneyError::Io(_)));
}

#[test]
fn test_journey_error_from_yaml_error() {
    // Create a YAML error by trying to parse invalid YAML
    let invalid_yaml = "invalid: yaml: content: [";
    let yaml_error = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml).unwrap_err();
    let journey_error: JourneyError = yaml_error.into();
    assert!(matches!(journey_error, JourneyError::Yaml(_)));
}

#[test]
fn test_journey_error_from_chrono_error() {
    // Create a chrono error by trying to parse invalid date
    let chrono_error = "invalid-date".parse::<chrono::NaiveDate>().unwrap_err();
    let journey_error: JourneyError = chrono_error.into();
    assert!(matches!(journey_error, JourneyError::DateTime(_)));
}

#[test]
fn test_vault_not_found_error() {
    let error = JourneyError::VaultNotFound("test-vault".to_string());
    assert!(error.to_string().contains("Vault not found"));
    assert!(error.to_string().contains("test-vault"));
}

#[test]
fn test_invalid_date_format_error() {
    let error = JourneyError::InvalidDateFormat("invalid".to_string());
    assert!(error.to_string().contains("Invalid date format"));
    assert!(error.to_string().contains("invalid"));
}

#[test]
fn test_invalid_time_format_error() {
    let error = JourneyError::InvalidTimeFormat("invalid".to_string());
    assert!(error.to_string().contains("Invalid time format"));
    assert!(error.to_string().contains("invalid"));
}

#[test]
fn test_editor_not_found_error() {
    let error = JourneyError::EditorNotFound("vim".to_string());
    assert!(error.to_string().contains("Editor not found"));
    assert!(error.to_string().contains("vim"));
}
