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

    /// Get locale-dependent table headers
    pub fn get_table_headers(&self) -> (String, String) {
        // Check for custom table headers first
        if let Some(ref custom_headers) = self.config.table_headers {
            return (custom_headers.time.clone(), custom_headers.content.clone());
        }
        
        let locale = &self.config.locale;
        
        // Check for Norwegian locale
        if locale.starts_with("no_") || locale.starts_with("nb_") || locale.starts_with("nn_") {
            ("Tid".to_string(), "Innhold".to_string())
        }
        // Check for Swedish locale
        else if locale.starts_with("sv_") {
            ("Tid".to_string(), "Innehåll".to_string())
        }
        // Check for Danish locale
        else if locale.starts_with("da_") {
            ("Tid".to_string(), "Indhold".to_string())
        }
        // Check for Finnish locale
        else if locale.starts_with("fi_") {
            ("Aika".to_string(), "Sisältö".to_string())
        }
        // Check for German locale
        else if locale.starts_with("de_") {
            ("Zeit".to_string(), "Inhalt".to_string())
        }
        // Check for French locale
        else if locale.starts_with("fr_") {
            ("Heure".to_string(), "Contenu".to_string())
        }
        // Check for Spanish locale
        else if locale.starts_with("es_") {
            ("Hora".to_string(), "Contenido".to_string())
        }
        // Check for Italian locale
        else if locale.starts_with("it_") {
            ("Ora".to_string(), "Contenuto".to_string())
        }
        // Check for Dutch locale
        else if locale.starts_with("nl_") {
            ("Tijd".to_string(), "Inhoud".to_string())
        }
        // Check for Portuguese locale
        else if locale.starts_with("pt_") {
            ("Hora".to_string(), "Conteúdo".to_string())
        }
        // Check for Russian locale
        else if locale.starts_with("ru_") {
            ("Время".to_string(), "Содержание".to_string())
        }
        // Check for Japanese locale
        else if locale.starts_with("ja_") {
            ("時間".to_string(), "内容".to_string())
        }
        // Check for Chinese locale
        else if locale.starts_with("zh_") {
            ("时间".to_string(), "内容".to_string())
        }
        // Default to English
        else {
            ("Time".to_string(), "Content".to_string())
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

    pub fn add_note(&self, content: &str, timestamp: Option<DateTime<Local>>, category: Option<&str>) -> Result<(), JourneyError> {
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
        let note_format = self.config.list_type.as_ref().unwrap_or(&NoteFormat::Bullet);
        let note_entry = self.format_note_entry(&formatted_time, &expanded_content, note_format);

        // Check if file exists and has content
        if note_path.exists() {
            let existing_content = fs::read_to_string(&note_path)?;
            
            // Check if we need to convert the existing format
            let converted_content = self.convert_note_format_if_needed(&existing_content, note_format)?;
            
            // If section_header is specified, find and append to that section
            if let Some(section_name) = self.config.get_section_header(category) {
                if let Some(section_start) = self.find_section(&converted_content, section_name) {
                    let mut lines: Vec<&str> = converted_content.lines().collect();
                    
                    // Find the actual end of content in the section
                    let content_end = self.find_section_content_end(&lines, section_start);
                    
                    // Insert the note at the end of the content in the section
                    lines.insert(content_end, &note_entry);
                    
                    fs::write(&note_path, lines.join("\n"))?;
                    return Ok(());
                } else {
                    // Section doesn't exist, create it at the end
                    let mut new_content = converted_content;
                    if !new_content.ends_with('\n') {
                        new_content.push('\n');
                    }
                    new_content.push_str(&format!("\n# {}\n", section_name));
                    new_content.push_str(&note_entry);
                    fs::write(&note_path, new_content)?;
                    return Ok(());
                }
            }
            
            // Append to end of file
            let mut content = converted_content;
            // note_entry already includes a newline, so we don't add an extra one
            content.push_str(&note_entry);
            fs::write(&note_path, content)?;
        } else {
            // Create new file
            let file_content = if let Some(template_file) = &self.config.template_file {
                // Use template file
                self.create_file_from_template(template_file, timestamp, &note_entry)?
            } else {
                // Use default template
                self.create_default_file_content(date, &note_entry, category)
            };
            
            fs::write(&note_path, file_content)?;
        }

        Ok(())
    }

    fn create_default_file_content(&self, date: NaiveDate, note_entry: &str, category: Option<&str>) -> String {
        let mut file_content = String::new();
        
        // Add frontmatter
        file_content.push_str("---\n");
        file_content.push_str(&format!("date: {}\n", self.date_handler.format_date(date)));
        file_content.push_str("---\n\n");
        
        // Add section if specified
        if let Some(section_name) = self.config.get_section_header(category) {
            file_content.push_str(&format!("# {}\n\n", section_name));
        }
        
        // Add table header if using table format
        let note_format = self.config.list_type.as_ref().unwrap_or(&NoteFormat::Bullet);
        if note_format == &NoteFormat::Table {
            let (time_header, content_header) = self.get_table_headers();
            file_content.push_str(&format!("| {} | {} |\n", time_header, content_header));
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
        if let Some(section_header) = &self.config.section_header {
            processed_content = processed_content.replace("{{section_header}}", section_header);
            processed_content = processed_content.replace("{section_header}", section_header);
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

    pub fn find_section_content_end(&self, lines: &[&str], section_start: usize) -> usize {
        let section_end = self.find_section_end(lines, section_start);
        
        // Find the actual end of content in the section (skip blank lines at the end)
        let mut content_end = section_start + 1;
        for i in section_start + 1..section_end {
            if !lines[i].trim().is_empty() {
                content_end = i + 1;
            }
        }
        
        content_end
    }

    pub fn list_notes(&self, date: NaiveDate, category: Option<&str>) -> Result<Vec<String>, JourneyError> {
        let note_path = self.get_note_path(date);
        
        if !note_path.exists() {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(&note_path)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut notes = Vec::new();
        
        // If section_header is configured, only list notes within that section
        if let Some(section_name) = self.config.get_section_header(category) {
            if let Some(section_start) = self.find_section(&content, section_name) {
                let section_end = self.find_section_end(&lines, section_start);
                
                // Only process lines within the section
                for line in &lines[section_start..section_end] {
                    let trimmed = line.trim();
                    // Check for bullet format
                    if trimmed.starts_with("- ") {
                        notes.push(line.to_string());
                    }
                    // Check for table format (but not table headers or separators)
                    else if trimmed.starts_with("|") && !trimmed.starts_with("|---") && trimmed.contains("|") {
                        // Skip if it looks like a table header
                        let (time_header, content_header) = self.get_table_headers();
                        if !trimmed.contains(&time_header) && !trimmed.contains(&content_header) && !trimmed.contains("Note") {
                            notes.push(line.to_string());
                        }
                    }
                }
            }
            // If section doesn't exist, return empty list
        } else {
            // No section configured, list all notes in the file
            for line in &lines {
                let trimmed = line.trim();
                // Check for bullet format
                if trimmed.starts_with("- ") {
                    notes.push(line.to_string());
                }
                // Check for table format (but not table headers or separators)
                else if trimmed.starts_with("|") && !trimmed.starts_with("|---") && trimmed.contains("|") {
                    // Skip if it looks like a table header
                    let (time_header, content_header) = self.get_table_headers();
                    if !trimmed.contains(&time_header) && !trimmed.contains(&content_header) && !trimmed.contains("Note") {
                        notes.push(line.to_string());
                    }
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
            NoteFormat::Bullet => format!("- {} {}\n", timestamp, content),
            NoteFormat::Table => format!("| {} | {} |\n", timestamp, content),
        }
    }

    /// Detect the current note format in the content
    pub fn detect_note_format(&self, content: &str) -> Option<NoteFormat> {
        let lines: Vec<&str> = content.lines().collect();
        
        // Look for bullet format
        let has_bullet_notes = lines.iter().any(|line| line.trim().starts_with("- "));
        
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
        
        // If no format detected or already matches target, clean up blank lines if it's table format
        if current_format.is_none() || current_format.as_ref() == Some(target_format) {
            if target_format == &NoteFormat::Table {
                return Ok(self.clean_table_blank_lines(content));
            }
            return Ok(content.to_string());
        }
        
        let lines: Vec<&str> = content.lines().collect();
        let mut converted_lines = Vec::new();
        let mut first_note_found = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            // Convert bullet to table
            if target_format == &NoteFormat::Table && trimmed.starts_with("- ") {
                // Add table header before the first note
                if !first_note_found {
                    let (time_header, content_header) = self.get_table_headers();
                    converted_lines.push(format!("| {} | {} |", time_header, content_header));
                    converted_lines.push("|------|----------|".to_string());
                    first_note_found = true;
                }
                
                if let Some((timestamp, note_content)) = self.parse_bullet_note(trimmed) {
                    converted_lines.push(format!("| {} | {} |", timestamp, note_content));
                } else {
                    converted_lines.push(line.to_string());
                }
            }
            // Skip blank lines when converting bullet to table
            else if target_format == &NoteFormat::Table && trimmed.is_empty() && first_note_found {
                // Skip blank lines between bullet points when converting to table
                continue;
            }
            // Convert table to bullet
            else if target_format == &NoteFormat::Bullet && trimmed.starts_with("|") && !trimmed.starts_with("|---") && !trimmed.contains("Time") && !trimmed.contains("Content") {
                if let Some((timestamp, note_content)) = self.parse_table_note(trimmed) {
                    converted_lines.push(format!("- {} {}", timestamp, note_content));
                } else {
                    converted_lines.push(line.to_string());
                }
            }
            // Skip table headers and separators when converting to bullet
            else if target_format == &NoteFormat::Bullet && trimmed.starts_with("|") && (trimmed.contains("Time") || trimmed.contains("Content") || trimmed.starts_with("|---")) {
                // Skip table headers and separators
                continue;
            }
            // Skip blank lines when converting table to bullet
            else if target_format == &NoteFormat::Bullet && trimmed.is_empty() {
                // Skip blank lines between table rows when converting to bullet
                continue;
            }
            else {
                converted_lines.push(line.to_string());
            }
        }
        
        Ok(converted_lines.join("\n"))
    }

    /// Clean up blank lines in table format
    fn clean_table_blank_lines(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut cleaned_lines = Vec::new();
        let mut in_table = false;
        let mut last_was_table_row = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            // Check if this is a table row
            if trimmed.starts_with("|") && !trimmed.starts_with("|---") && !trimmed.contains("Time") && !trimmed.contains("Content") {
                in_table = true;
                last_was_table_row = true;
                cleaned_lines.push(line.to_string());
            }
            // Check if this is a table header or separator
            else if trimmed.starts_with("|") && (trimmed.contains("Time") || trimmed.contains("Content") || trimmed.starts_with("|---")) {
                in_table = true;
                last_was_table_row = false;
                cleaned_lines.push(line.to_string());
            }
            // Check if this is a blank line
            else if trimmed.is_empty() {
                // Only keep blank lines if we're not in a table or if it's the first blank line after a table
                if !in_table || !last_was_table_row {
                    cleaned_lines.push(line.to_string());
                }
                last_was_table_row = false;
            }
            // Non-table content
            else {
                in_table = false;
                last_was_table_row = false;
                cleaned_lines.push(line.to_string());
            }
        }
        
        cleaned_lines.join("\n")
    }

    /// Parse a bullet note to extract timestamp and content
    fn parse_bullet_note(&self, line: &str) -> Option<(String, String)> {
        // Format: "- timestamp content"
        if let Some(start) = line.find("- ") {
            let after_dash = &line[start + 2..];
            // Find the first space after the dash to separate timestamp from content
            if let Some(space_pos) = after_dash.find(' ') {
                let timestamp = after_dash[..space_pos].to_string();
                let content = after_dash[space_pos + 1..].trim().to_string();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn create_test_vault() -> (Vault, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let vault_path = temp_dir.path().join("test_vault");
        fs::create_dir_all(&vault_path).unwrap();

        let config = VaultConfig {
            name: "test".to_string(),
            path: vault_path,
            locale: "en_US.UTF-8".to_string(),
            phrases: HashMap::new(),
            section_header: None,
            section_header_work: None,
            section_header_personal: None,
            section_header_health: None,
            section_header_meetings: None,
            table_headers: None,
            date_format: None,
            template_file: None,
            file_path_format: None,
            list_type: Some(NoteFormat::Table),
        };

        let vault = Vault::new(config);
        (vault, temp_dir)
    }

    #[test]
    fn test_table_format_no_blank_lines() {
        let (vault, _temp_dir) = create_test_vault();
        let date = chrono::Local::now().date_naive();

        // Add first note
        vault.add_note("Test1", None, None).unwrap();

        // Add second note
        vault.add_note("Test2", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        

        // Find the table section
        let mut table_start = None;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("| Time | Content |") {
                table_start = Some(i);
                break;
            }
        }

        assert!(table_start.is_some(), "Table header not found");
        let start = table_start.unwrap();

        // Check that there are no blank lines between table rows
        let table_lines = &lines[start..];
        
        // Find the first data row (after header and separator)
        let mut data_start = None;
        for (i, line) in table_lines.iter().enumerate() {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                data_start = Some(i);
                break;
            }
        }

        assert!(data_start.is_some(), "No data rows found in table");
        let data_start = data_start.unwrap();

        // Check that there are no blank lines between data rows
        let mut in_data_section = false;
        let mut blank_line_found = false;
        
        for line in &table_lines[data_start..] {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                if in_data_section && blank_line_found {
                    panic!("Blank line found between table rows");
                }
                in_data_section = true;
                blank_line_found = false;
            } else if line.trim().is_empty() {
                if in_data_section {
                    blank_line_found = true;
                }
            }
        }
    }

    #[test]
    fn test_table_format_existing_file_no_blank_lines() {
        let (vault, _temp_dir) = create_test_vault();
        let date = chrono::Local::now().date_naive();

        // Create a file with some existing content first that has blank lines
        let note_path = vault.get_note_path(date);
        let existing_content = "---
date: 2025-10-26
---

| Time | Content |
|------|----------|
| 00:04:43 | Test1 |

| 00:05:28 | Test2 |
";
        fs::write(&note_path, existing_content).unwrap();

        // Add third note to existing file
        vault.add_note("Test3", None, None).unwrap();

        // Add fourth note to existing file
        vault.add_note("Test4", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        

        // Find the table section
        let mut table_start = None;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("| Time | Content |") {
                table_start = Some(i);
                break;
            }
        }

        assert!(table_start.is_some(), "Table header not found");
        let start = table_start.unwrap();

        // Check that there are no blank lines between table rows
        let table_lines = &lines[start..];
        
        // Find the first data row (after header and separator)
        let mut data_start = None;
        for (i, line) in table_lines.iter().enumerate() {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                data_start = Some(i);
                break;
            }
        }

        assert!(data_start.is_some(), "No data rows found in table");
        let data_start = data_start.unwrap();

        // Check that there are no blank lines between data rows
        let mut in_data_section = false;
        let mut blank_line_found = false;
        
        for line in &table_lines[data_start..] {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                if in_data_section && blank_line_found {
                    panic!("Blank line found between table rows in existing file");
                }
                in_data_section = true;
                blank_line_found = false;
            } else if line.trim().is_empty() {
                if in_data_section {
                    blank_line_found = true;
                }
            }
        }
    }

    #[test]
    fn test_bullet_format_no_header() {
        let (mut vault, _temp_dir) = create_test_vault();
        
        // Set bullet format
        vault.config.list_type = Some(NoteFormat::Bullet);
        
        let date = chrono::Local::now().date_naive();

        // Add first note - should not create a header
        vault.add_note("Test1", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        
        
        // Should not contain table headers
        assert!(!content.contains("| Time | Content |"), "Bullet format should not have table headers");
        assert!(!content.contains("|------|----------|"), "Bullet format should not have table separators");
        
        // Should contain bullet format
        assert!(content.contains("- "), "Bullet format should contain bullet points");
    }

    #[test]
    fn test_table_to_bullet_conversion_removes_header() {
        let (mut vault, _temp_dir) = create_test_vault();
        
        // Start with table format
        vault.config.list_type = Some(NoteFormat::Table);
        
        let date = chrono::Local::now().date_naive();

        // Add first note to create table format
        vault.add_note("Test1", None, None).unwrap();

        // Switch to bullet format
        vault.config.list_type = Some(NoteFormat::Bullet);

        // Add second note - should convert to bullet format without headers
        vault.add_note("Test2", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        
        
        // Should not contain table headers
        assert!(!content.contains("| Time | Content |"), "Converted bullet format should not have table headers");
        assert!(!content.contains("|------|----------|"), "Converted bullet format should not have table separators");
        
        // Should contain bullet format
        assert!(content.contains("- "), "Converted bullet format should contain bullet points");
    }

    #[test]
    fn test_bullet_to_table_conversion_no_blank_lines() {
        let (mut vault, _temp_dir) = create_test_vault();
        
        // Start with bullet format
        vault.config.list_type = Some(NoteFormat::Bullet);
        
        let date = chrono::Local::now().date_naive();

        // Create a file with bullet points that have blank lines between them
        let note_path = vault.get_note_path(date);
        let existing_content = "---
date: 2025-10-26
---

- 00:04:43 Test1

- 00:05:28 Test2
";
        fs::write(&note_path, existing_content).unwrap();

        // Switch to table format
        vault.config.list_type = Some(NoteFormat::Table);

        // Add third note - should convert to table format without blank lines
        vault.add_note("Test3", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        
        
        // Should not contain blank lines between table rows
        let lines: Vec<&str> = content.lines().collect();
        let mut in_table = false;
        let mut blank_line_found = false;
        
        for line in &lines {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                if in_table && blank_line_found {
                    panic!("Blank line found between table rows in bullet-to-table conversion");
                }
                in_table = true;
                blank_line_found = false;
            } else if line.trim().is_empty() {
                if in_table {
                    blank_line_found = true;
                }
            }
        }
    }

    #[test]
    fn test_table_to_bullet_conversion_no_blank_lines() {
        let (mut vault, _temp_dir) = create_test_vault();
        
        // Start with table format
        vault.config.list_type = Some(NoteFormat::Table);
        
        let date = chrono::Local::now().date_naive();

        // Create a file with table rows that have blank lines between them
        let note_path = vault.get_note_path(date);
        let existing_content = "---
date: 2025-10-26
---

| Time | Content |
|------|----------|
| 00:04:43 | Test1 |

| 00:05:28 | Test2 |
";
        fs::write(&note_path, existing_content).unwrap();

        // Switch to bullet format
        vault.config.list_type = Some(NoteFormat::Bullet);

        // Add third note - should convert to bullet format without blank lines
        vault.add_note("Test3", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        
        
        // Should not contain blank lines between bullet points
        let lines: Vec<&str> = content.lines().collect();
        let mut in_bullet_section = false;
        let mut blank_line_found = false;
        
        for line in &lines {
            if line.starts_with("- ") {
                if in_bullet_section && blank_line_found {
                    panic!("Blank line found between bullet points in table-to-bullet conversion");
                }
                in_bullet_section = true;
                blank_line_found = false;
            } else if line.trim().is_empty() {
                if in_bullet_section {
                    blank_line_found = true;
                }
            }
        }
    }

    #[test]
    fn test_adding_note_to_existing_file_no_blank_lines() {
        let (vault, _temp_dir) = create_test_vault();
        let date = chrono::Local::now().date_naive();

        // Create a file with some existing content that has blank lines between table rows
        let note_path = vault.get_note_path(date);
        let existing_content = "---
date: 2025-10-26
---

| Time | Content |
|------|----------|
| 00:04:43 | Test1 |

| 00:05:28 | Test2 |
";
        fs::write(&note_path, existing_content).unwrap();

        // Add third note to existing file - should not create blank lines
        vault.add_note("Test3", None, None).unwrap();

        // Add fourth note to existing file - should not create blank lines
        vault.add_note("Test4", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        
        // Debug: print the actual content
        println!("Adding notes to existing file content:");
        println!("{}", content);
        
        // Should not contain blank lines between notes
        let lines: Vec<&str> = content.lines().collect();
        let mut in_notes_section = false;
        let mut blank_line_found = false;
        
        for line in &lines {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                if in_notes_section && blank_line_found {
                    panic!("Blank line found between notes when adding to existing file");
                }
                in_notes_section = true;
                blank_line_found = false;
            } else if line.trim().is_empty() {
                if in_notes_section {
                    blank_line_found = true;
                }
            }
        }
    }

    #[test]
    fn test_append_note_without_conversion_no_blank_lines() {
        let (vault, _temp_dir) = create_test_vault();
        let date = chrono::Local::now().date_naive();

        // Create a file with some existing content
        let note_path = vault.get_note_path(date);
        let existing_content = "---
date: 2025-10-26
---

| Time | Content |
|------|----------|
| 00:04:43 | Test1 |
| 00:05:28 | Test2 |
";
        fs::write(&note_path, existing_content).unwrap();

        // Add third note to existing file - this should trigger the append logic
        vault.add_note("Test3", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        
        
        // Should not contain blank lines between notes
        let lines: Vec<&str> = content.lines().collect();
        let mut in_notes_section = false;
        let mut blank_line_found = false;
        
        for line in &lines {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                if in_notes_section && blank_line_found {
                    panic!("Blank line found between notes when appending to existing file");
                }
                in_notes_section = true;
                blank_line_found = false;
            } else if line.trim().is_empty() {
                if in_notes_section {
                    blank_line_found = true;
                }
            }
        }
    }

    #[test]
    fn test_section_insertion_no_blank_lines() {
        let (mut vault, _temp_dir) = create_test_vault();
        
        // Set up a section header
        vault.config.section_header = Some("Test Section".to_string());
        
        let date = chrono::Local::now().date_naive();

        // Create a file with a section that has some content
        let note_path = vault.get_note_path(date);
        let existing_content = "---
date: 2025-10-26
---

# Test Section

| Time | Content |
|------|----------|
| 00:04:43 | Test1 |
| 00:05:28 | Test2 |

# Another Section
Some other content
";
        fs::write(&note_path, existing_content).unwrap();

        // Add third note to the section - should not create blank lines
        vault.add_note("Test3", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        
        // Debug: print the actual content
        println!("Section insertion content:");
        println!("{}", content);
        
        // Should not contain blank lines between notes in the section
        let lines: Vec<&str> = content.lines().collect();
        let mut in_section = false;
        let mut blank_line_found = false;
        
        for line in &lines {
            if *line == "# Test Section" {
                in_section = true;
                blank_line_found = false;
                continue;
            }
            if *line == "# Another Section" {
                in_section = false;
                break;
            }
            if in_section && line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                if blank_line_found {
                    panic!("Blank line found between notes in section");
                }
                blank_line_found = false;
            } else if in_section && line.trim().is_empty() {
                blank_line_found = true;
            }
        }
    }

    #[test]
    fn test_header_flag_functionality() {
        let (vault, _temp_dir) = create_test_vault();
        let date = chrono::Local::now().date_naive();

        // Add some notes in table format
        vault.add_note("Test1", None, None).unwrap();
        vault.add_note("Test2", None, None).unwrap();

        // Test listing without header flag
        let notes = vault.list_notes(date, None).unwrap();
        assert!(!notes.is_empty(), "Should have notes");
        
        // Verify that table headers are not included in the notes
        for note in &notes {
            assert!(!note.contains("Time") && !note.contains("Content"), 
                "Table headers should not be included in notes: {}", note);
        }
    }

    #[test]
    fn test_table_format_with_section_no_blank_lines() {
        let (mut vault, _temp_dir) = create_test_vault();
        
        // Set up a section header
        vault.config.section_header = Some("Test Section".to_string());
        
        let date = chrono::Local::now().date_naive();

        // Add first note
        vault.add_note("Test1", None, None).unwrap();

        // Add second note
        vault.add_note("Test2", None, None).unwrap();

        // Read the file content
        let note_path = vault.get_note_path(date);
        let content = fs::read_to_string(&note_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // Find the section
        let mut section_start = None;
        for (i, line) in lines.iter().enumerate() {
            if *line == "# Test Section" {
                section_start = Some(i);
                break;
            }
        }

        assert!(section_start.is_some(), "Section header not found");
        let start = section_start.unwrap();

        // Check that there are no blank lines between the section header and table
        let section_lines = &lines[start..];
        
        // The line after the section header should be the table header
        assert!(section_lines.len() > 1, "No content after section header");
        
        // Find the table header after the section
        let mut table_header_found = false;
        for line in section_lines {
            if line.starts_with("| Time | Content |") {
                table_header_found = true;
                break;
            }
        }
        assert!(table_header_found, "Table header not found after section");

        // Check for blank lines between table rows
        let mut in_table = false;
        let mut blank_line_found = false;
        
        for line in section_lines {
            if line.starts_with("|") && !line.starts_with("|---") && !line.contains("Time") && !line.contains("Content") {
                if in_table && blank_line_found {
                    panic!("Blank line found between table rows in section");
                }
                in_table = true;
                blank_line_found = false;
            } else if line.trim().is_empty() {
                if in_table {
                    blank_line_found = true;
                }
            }
        }
    }
}

