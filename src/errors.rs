use thiserror::Error;

#[derive(Error, Debug)]
pub enum JourneyError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Date/time parsing error: {0}")]
    DateTime(#[from] chrono::ParseError),

    #[error("Vault not found: {0}")]
    VaultNotFound(String),

    #[error("Invalid date format: {0}")]
    InvalidDateFormat(String),

    #[error("Invalid time format: {0}")]
    InvalidTimeFormat(String),

    #[error("Editor not found: {0}")]
    EditorNotFound(String),
}

