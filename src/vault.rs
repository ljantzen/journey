use crate::config::VaultConfig;
use crate::date_time::DateTimeHandler;
use crate::errors::JourneyError;
use chrono::{DateTime, Local, NaiveDate};
use std::fs;
use std::path::PathBuf;

pub struct Vault {
    pub config: VaultConfig,
    pub date_handler: DateTimeHandler,
}

impl Vault {
    pub fn new(config: VaultConfig) -> Self {
        let date_handler = DateTimeHandler::new(config.locale.clone());
        Self {
            config,
            date_handler,
        }
    }

    pub fn get_note_path(&self, date: NaiveDate) -> PathBuf {
        let date_str = self.date_handler.format_date(date);
        self.config.path.join(format!("{}.md", date_str))
    }

    pub fn add_note(&self, content: &str, timestamp: Option<DateTime<Local>>) -> Result<(), JourneyError> {
        let timestamp = timestamp.unwrap_or_else(|| self.date_handler.get_current_datetime());
        let date = timestamp.date_naive();
        let note_path = self.get_note_path(date);

        // Ensure the vault directory exists
        if let Some(parent) = note_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let formatted_time = self.date_handler.format_datetime(timestamp);
        let note_entry = format!("- [{}] {}\n", formatted_time, content);

        // Check if file exists and has content
        if note_path.exists() {
            let existing_content = fs::read_to_string(&note_path)?;
            
            // If section_name is specified, find and append to that section
            if let Some(section_name) = &self.config.section_name {
                if let Some(section_start) = self.find_section(&existing_content, section_name) {
                    let mut lines: Vec<&str> = existing_content.lines().collect();
                    lines.insert(section_start + 1, &note_entry);
                    fs::write(&note_path, lines.join("\n"))?;
                    return Ok(());
                }
            }
            
            // Append to end of file
            let mut content = existing_content;
            if !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(&note_entry);
            fs::write(&note_path, content)?;
        } else {
            // Create new file
            let mut file_content = String::new();
            
            // Add frontmatter
            file_content.push_str("---\n");
            file_content.push_str(&format!("date: {}\n", self.date_handler.format_date(date)));
            file_content.push_str("---\n\n");
            
            // Add section if specified
            if let Some(section_name) = &self.config.section_name {
                file_content.push_str(&format!("# {}\n\n", section_name));
            }
            
            file_content.push_str(&note_entry);
            fs::write(&note_path, file_content)?;
        }

        Ok(())
    }

    pub fn find_section(&self, content: &str, section_name: &str) -> Option<usize> {
        let lines: Vec<&str> = content.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.trim().starts_with('#') && line.contains(section_name) {
                return Some(i);
            }
        }
        None
    }

    pub fn list_notes(&self, date: NaiveDate) -> Result<Vec<String>, JourneyError> {
        let note_path = self.get_note_path(date);
        
        if !note_path.exists() {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(&note_path)?;
        let mut notes = Vec::new();
        
        for line in content.lines() {
            if line.trim().starts_with("- [") {
                notes.push(line.to_string());
            }
        }

        Ok(notes)
    }

    pub fn get_editor_path(&self, date: NaiveDate) -> PathBuf {
        self.get_note_path(date)
    }
}

