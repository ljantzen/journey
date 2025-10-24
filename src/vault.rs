use crate::config::VaultConfig;
use crate::date_time::DateTimeHandler;
use crate::errors::JourneyError;
use chrono::{DateTime, Local, NaiveDate, Datelike, Weekday};
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
        if let Some(ref format) = self.config.file_path_format {
            // Use custom file path format
            let formatted_path = self.format_custom_path(format, date);
            self.config.path.join(formatted_path)
        } else {
            // Use default format: YYYY-MM-DD.md
            let date_str = self.date_handler.format_date(date);
            self.config.path.join(format!("{}.md", date_str))
        }
    }

    pub fn add_note(&self, content: &str, timestamp: Option<DateTime<Local>>) -> Result<(), JourneyError> {
        let timestamp = timestamp.unwrap_or_else(|| self.date_handler.get_current_datetime());
        let date = timestamp.date_naive();
        let note_path = self.get_note_path(date);

        // Ensure the vault directory exists
        if let Some(parent) = note_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Expand phrases in the content
        let expanded_content = self.expand_phrases(content);
        let formatted_time = self.date_handler.format_datetime(timestamp);
        let note_entry = format!("- [{}] {}\n", formatted_time, expanded_content);

        // Check if file exists and has content
        if note_path.exists() {
            let existing_content = fs::read_to_string(&note_path)?;
            
            // If section_name is specified, find and append to that section
            if let Some(section_name) = &self.config.section_name {
                if let Some(section_start) = self.find_section(&existing_content, section_name) {
                    let mut lines: Vec<&str> = existing_content.lines().collect();
                    
                    // Find the end of the section (next section or end of file)
                    let section_end = self.find_section_end(&lines, section_start);
                    
                    // Insert the note at the end of the section
                    lines.insert(section_end, &note_entry);
                    
                    fs::write(&note_path, lines.join("\n"))?;
                    return Ok(());
                } else {
                    // Section doesn't exist, create it at the end
                    let mut new_content = existing_content;
                    if !new_content.ends_with('\n') {
                        new_content.push('\n');
                    }
                    new_content.push_str(&format!("\n# {}\n\n", section_name));
                    new_content.push_str(&note_entry);
                    fs::write(&note_path, new_content)?;
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
            let file_content = if let Some(template_file) = &self.config.template_file {
                // Use template file
                self.create_file_from_template(template_file, timestamp, &note_entry)?
            } else {
                // Use default template
                self.create_default_file_content(date, &note_entry)
            };
            
            fs::write(&note_path, file_content)?;
        }

        Ok(())
    }

    fn create_default_file_content(&self, date: NaiveDate, note_entry: &str) -> String {
        let mut file_content = String::new();
        
        // Add frontmatter
        file_content.push_str("---\n");
        file_content.push_str(&format!("date: {}\n", self.date_handler.format_date(date)));
        file_content.push_str("---\n\n");
        
        // Add section if specified
        if let Some(section_name) = &self.config.section_name {
            file_content.push_str(&format!("# {}\n\n", section_name));
        }
        
        file_content.push_str(note_entry);
        file_content
    }

    fn create_file_from_template(&self, template_file: &str, timestamp: DateTime<Local>, note_entry: &str) -> Result<String, JourneyError> {
        // Read the template file
        let template_path = PathBuf::from(template_file);
        let template_content = fs::read_to_string(&template_path)
            .map_err(|e| JourneyError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Failed to read template file '{}': {}", template_file, e))))?;
        
        // Process template variables
        let mut processed_content = template_content;
        
        // Replace template variables
        let date = timestamp.date_naive();
        processed_content = processed_content.replace("{{date}}", &self.date_handler.format_date(date));
        processed_content = processed_content.replace("{{time}}", &self.date_handler.format_time(timestamp.time()));
        processed_content = processed_content.replace("{{datetime}}", &self.date_handler.format_datetime(timestamp));
        
        // Handle section name replacement
        if let Some(section_name) = &self.config.section_name {
            processed_content = processed_content.replace("{{section_name}}", section_name);
        }
        
        // If the template doesn't contain a placeholder for notes, append the note
        if !processed_content.contains("{{note}}") {
            processed_content.push_str(note_entry);
        } else {
            processed_content = processed_content.replace("{{note}}", note_entry);
        }
        
        Ok(processed_content)
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

    pub fn find_section_end(&self, lines: &[&str], section_start: usize) -> usize {
        // Look for the next section header or end of file
        for i in (section_start + 1)..lines.len() {
            if lines[i].trim().starts_with('#') {
                return i;
            }
        }
        // If no next section found, return end of file
        lines.len()
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

    /// Expand phrases in the content using the vault's phrase mappings
    fn expand_phrases(&self, content: &str) -> String {
        let mut result = content.to_string();
        
        // Sort phrases by length (longest first) to avoid partial replacements
        let mut phrases: Vec<_> = self.config.phrases.iter().collect();
        phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        
        for (phrase, replacement) in phrases {
            // Replace all occurrences of the phrase with its replacement
            result = result.replace(phrase, replacement);
        }
        
        result
    }

    /// Format a custom file path using date components
    fn format_custom_path(&self, format: &str, date: NaiveDate) -> String {
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let weekday = date.weekday();
        
        let mut result = format.to_string();
        
        // Replace year
        result = result.replace("{year}", &year.to_string());
        
        // Replace month with zero-padding
        result = result.replace("{month:02}", &format!("{:02}", month));
        result = result.replace("{month}", &month.to_string());
        
        // Replace day/date with zero-padding
        result = result.replace("{day:02}", &format!("{:02}", day));
        result = result.replace("{date:02}", &format!("{:02}", day));
        result = result.replace("{day}", &day.to_string());
        result = result.replace("{date}", &day.to_string());
        
        // Replace weekday names (case-sensitive)
        result = result.replace("{Weekday}", &self.format_weekday(weekday, false));
        result = result.replace("{weekday}", &self.format_weekday(weekday, false).to_lowercase());
        result = result.replace("{Weekday_short}", &self.format_weekday(weekday, true));
        result = result.replace("{weekday_short}", &self.format_weekday(weekday, true).to_lowercase());
        
        // Replace month names (case-sensitive)
        result = result.replace("{Month}", &self.format_month(month, false));
        result = result.replace("{month_name}", &self.format_month(month, false).to_lowercase());
        result = result.replace("{Month_short}", &self.format_month(month, true));
        result = result.replace("{month_short}", &self.format_month(month, true).to_lowercase());
        
        result
    }
    
    /// Format weekday name (full or short)
    fn format_weekday(&self, weekday: Weekday, short: bool) -> String {
        match weekday {
            Weekday::Mon => if short { "Mon".to_string() } else { "Monday".to_string() },
            Weekday::Tue => if short { "Tue".to_string() } else { "Tuesday".to_string() },
            Weekday::Wed => if short { "Wed".to_string() } else { "Wednesday".to_string() },
            Weekday::Thu => if short { "Thu".to_string() } else { "Thursday".to_string() },
            Weekday::Fri => if short { "Fri".to_string() } else { "Friday".to_string() },
            Weekday::Sat => if short { "Sat".to_string() } else { "Saturday".to_string() },
            Weekday::Sun => if short { "Sun".to_string() } else { "Sunday".to_string() },
        }
    }
    
    /// Format month name (full or short)
    fn format_month(&self, month: u32, short: bool) -> String {
        match month {
            1 => if short { "Jan".to_string() } else { "January".to_string() },
            2 => if short { "Feb".to_string() } else { "February".to_string() },
            3 => if short { "Mar".to_string() } else { "March".to_string() },
            4 => if short { "Apr".to_string() } else { "April".to_string() },
            5 => if short { "May".to_string() } else { "May".to_string() },
            6 => if short { "Jun".to_string() } else { "June".to_string() },
            7 => if short { "Jul".to_string() } else { "July".to_string() },
            8 => if short { "Aug".to_string() } else { "August".to_string() },
            9 => if short { "Sep".to_string() } else { "September".to_string() },
            10 => if short { "Oct".to_string() } else { "October".to_string() },
            11 => if short { "Nov".to_string() } else { "November".to_string() },
            12 => if short { "Dec".to_string() } else { "December".to_string() },
            _ => "Unknown".to_string(),
        }
    }
}

