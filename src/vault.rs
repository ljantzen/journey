use crate::config::{VaultConfig, NoteFormat};
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
        
        // Get the configured note format (default to bullet if not specified)
        let note_format = self.config.note_format.as_ref().unwrap_or(&NoteFormat::Bullet);
        let note_entry = self.format_note_entry(&formatted_time, &expanded_content, note_format);

        // Check if file exists and has content
        if note_path.exists() {
            let existing_content = fs::read_to_string(&note_path)?;
            
            // Check if we need to convert the existing format
            let converted_content = self.convert_note_format_if_needed(&existing_content, note_format)?;
            
            // If section_name is specified, find and append to that section
            if let Some(section_name) = &self.config.section_name {
                if let Some(section_start) = self.find_section(&converted_content, section_name) {
                    let mut lines: Vec<&str> = converted_content.lines().collect();
                    
                    // Find the end of the section (next section or end of file)
                    let section_end = self.find_section_end(&lines, section_start);
                    
                    // Insert the note at the end of the section
                    lines.insert(section_end, &note_entry);
                    
                    fs::write(&note_path, lines.join("\n"))?;
                    return Ok(());
                } else {
                    // Section doesn't exist, create it at the end
                    let mut new_content = converted_content;
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
            let mut content = converted_content;
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
        
        // Add table header if using table format
        let note_format = self.config.note_format.as_ref().unwrap_or(&NoteFormat::Bullet);
        if note_format == &NoteFormat::Table {
            file_content.push_str("| Time | Content |\n");
            file_content.push_str("|------|----------|\n");
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
        
        // Additional template variables
        // Yesterday and tomorrow dates - use same format as note filenames
        let yesterday_date = date - chrono::Duration::days(1);
        let tomorrow_date = date + chrono::Duration::days(1);
        let yesterday = self.date_handler.format_date(yesterday_date);
        let tomorrow = self.date_handler.format_date(tomorrow_date);
        processed_content = processed_content.replace("{yesterday}", &yesterday);
        processed_content = processed_content.replace("{tomorrow}", &tomorrow);
        processed_content = processed_content.replace("{{yesterday}}", &yesterday);
        processed_content = processed_content.replace("{{tomorrow}}", &tomorrow);
        
        // Weekday (lowercase) - full weekday name
        let weekday = date.format("%A").to_string();
        processed_content = processed_content.replace("{weekday}", &weekday);
        processed_content = processed_content.replace("{{weekday}}", &weekday);
        
        // Weekday (uppercase) - abbreviated weekday name
        let weekday_short = date.format("%a").to_string();
        processed_content = processed_content.replace("{Weekday}", &weekday_short);
        processed_content = processed_content.replace("{{Weekday}}", &weekday_short);
        
        // Additional variables
        // Created timestamp
        let created = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
        processed_content = processed_content.replace("{created}", &created);
        processed_content = processed_content.replace("{{created}}", &created);
        
        // Today's date
        let today = self.date_handler.format_date(date);
        processed_content = processed_content.replace("{today}", &today);
        processed_content = processed_content.replace("{{today}}", &today);
        
        // Handle section name replacement
        if let Some(section_name) = &self.config.section_name {
            processed_content = processed_content.replace("{{section_name}}", section_name);
            processed_content = processed_content.replace("{section_name}", section_name);
        }
        
        // If the template doesn't contain a placeholder for notes, append the note
        if !processed_content.contains("{{note}}") && !processed_content.contains("{note}") {
            processed_content.push_str(note_entry);
        } else {
            processed_content = processed_content.replace("{{note}}", note_entry);
            processed_content = processed_content.replace("{note}", note_entry);
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
            let trimmed = line.trim();
            // Check for bullet format
            if trimmed.starts_with("- [") {
                notes.push(line.to_string());
            }
            // Check for table format (but not table headers or separators)
            else if trimmed.starts_with("|") && !trimmed.starts_with("|---") && trimmed.contains("|") {
                // Skip if it looks like a table header
                if !trimmed.contains("Time") && !trimmed.contains("Content") && !trimmed.contains("Note") {
                    notes.push(line.to_string());
                }
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
        
        // Replace date:MM (zero-padded month) for Journals plugin compatibility
        // Process double braces first to avoid conflicts
        result = result.replace("{{date:MM}}", &format!("{:02}", month));
        result = result.replace("{date:MM}", &format!("{:02}", month));
        
        // Replace date:y (two-digit year) for Journals plugin compatibility
        // Process double braces first to avoid conflicts
        result = result.replace("{{date:y}}", &format!("{:02}", year % 100));
        result = result.replace("{date:y}", &format!("{:02}", year % 100));
        
        // Replace day/date with zero-padding (after date:MM and date:y to avoid conflicts)
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

    /// Format a note entry according to the specified format
    fn format_note_entry(&self, timestamp: &str, content: &str, format: &NoteFormat) -> String {
        match format {
            NoteFormat::Bullet => format!("- [{}] {}\n", timestamp, content),
            NoteFormat::Table => format!("| {} | {} |\n", timestamp, content),
        }
    }

    /// Detect the current note format in the content
    pub fn detect_note_format(&self, content: &str) -> Option<NoteFormat> {
        let lines: Vec<&str> = content.lines().collect();
        
        // Look for bullet format
        let has_bullet_notes = lines.iter().any(|line| line.trim().starts_with("- ["));
        
        // Look for table format
        let has_table_notes = lines.iter().any(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("|") && trimmed.contains("|") && !trimmed.starts_with("|---")
        });
        
        if has_bullet_notes && !has_table_notes {
            Some(NoteFormat::Bullet)
        } else if has_table_notes && !has_bullet_notes {
            Some(NoteFormat::Table)
        } else {
            None // Mixed or no notes found
        }
    }

    /// Convert note format if needed
    fn convert_note_format_if_needed(&self, content: &str, target_format: &NoteFormat) -> Result<String, JourneyError> {
        let current_format = self.detect_note_format(content);
        
        // If no format detected or already matches target, return as-is
        if current_format.is_none() || current_format.as_ref() == Some(target_format) {
            return Ok(content.to_string());
        }
        
        let lines: Vec<&str> = content.lines().collect();
        let mut converted_lines = Vec::new();
        let mut first_note_found = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            // Convert bullet to table
            if target_format == &NoteFormat::Table && trimmed.starts_with("- [") {
                // Add table header before the first note
                if !first_note_found {
                    converted_lines.push("| Time | Content |".to_string());
                    converted_lines.push("|------|----------|".to_string());
                    first_note_found = true;
                }
                
                if let Some((timestamp, note_content)) = self.parse_bullet_note(trimmed) {
                    converted_lines.push(format!("| {} | {} |", timestamp, note_content));
                } else {
                    converted_lines.push(line.to_string());
                }
            }
            // Convert table to bullet
            else if target_format == &NoteFormat::Bullet && trimmed.starts_with("|") && !trimmed.starts_with("|---") {
                if let Some((timestamp, note_content)) = self.parse_table_note(trimmed) {
                    converted_lines.push(format!("- [{}] {}", timestamp, note_content));
                } else {
                    converted_lines.push(line.to_string());
                }
            }
            else {
                converted_lines.push(line.to_string());
            }
        }
        
        Ok(converted_lines.join("\n"))
    }

    /// Parse a bullet note to extract timestamp and content
    fn parse_bullet_note(&self, line: &str) -> Option<(String, String)> {
        // Format: "- [timestamp] content"
        if let Some(start) = line.find("- [") {
            if let Some(end) = line[start + 3..].find("]") {
                let timestamp = line[start + 3..start + 3 + end].to_string();
                let content = line[start + 3 + end + 1..].trim().to_string();
                return Some((timestamp, content));
            }
        }
        None
    }

    /// Parse a table note to extract timestamp and content
    fn parse_table_note(&self, line: &str) -> Option<(String, String)> {
        // Format: "| timestamp | content |"
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 3 {
            let timestamp = parts[1].trim().to_string();
            let content = parts[2].trim().to_string();
            return Some((timestamp, content));
        }
        None
    }
}

