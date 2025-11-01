use crate::cli::Cli;
use crate::config::Config;
use crate::config_manager::ConfigManager;
use crate::errors::JourneyError;
use crate::vault::Vault;
use chrono::{Local, NaiveDate};
use std::env;
use std::process::Command;

#[derive(Debug, Clone)]
struct DailyNotesConfig {
    enabled: bool,
    format: String,
    folder: String,
    template: Option<String>,
}

#[derive(Debug, Clone)]
struct PeriodicNotesConfig {
    enabled: bool,
}

#[derive(Debug, Clone)]
struct JournalsConfig {
    name: String,
}

#[derive(Debug, Clone)]
struct ObsidianPluginConfigs {
    daily_notes: Option<DailyNotesConfig>,
    periodic_notes: Option<PeriodicNotesConfig>,
    journals: Vec<JournalsConfig>,
}

impl ObsidianPluginConfigs {
    fn new() -> Self {
        Self {
            daily_notes: None,
            periodic_notes: None,
            journals: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct CliArgs {
    pub vault: Option<String>,
    pub date: Option<String>,
    pub relative_date: Option<i64>,
    pub time: Option<String>,
    pub time_format: Option<String>,
}

pub struct App {
    config_manager: ConfigManager,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self, JourneyError> {
        let config_manager = ConfigManager::new()?;
        
        // Strictly require the config file to exist; surface a clear error otherwise
        if !config_manager.config_exists() {
            return Err(JourneyError::Config(
                "No configuration file found. Please run 'journey --init --path <path>' to create your first vault.".to_string()
            ));
        }
        let config = config_manager.load_config()?;
        
        Ok(Self {
            config_manager,
            config,
        })
    }

    pub fn new_for_init() -> Result<Self, JourneyError> {
        let config_manager = ConfigManager::new()?;
        let config = config_manager.load_config()?;
        
        Ok(Self {
            config_manager,
            config,
        })
    }

    pub fn run(&mut self, cli: Cli) -> Result<(), JourneyError> {
        match cli.command {
            Some(cmd) => {
                // Extract the parts we need before moving
                let vault = cli.vault.clone();
                let date = cli.date.clone();
                let relative_date = cli.relative_date;
                let time = cli.time.clone();
                let time_format = cli.time_format.clone();
                self.handle_command_with_args(cmd, vault, date, relative_date, time, time_format)
            }
            None => self.handle_default_behavior(&cli),
        }
    }

    pub fn run_journeyctl(&mut self, cli: crate::journeyctl::JourneyCtlCli) -> Result<(), JourneyError> {
        match cli.command {
            Some(cmd) => self.run_journeyctl_command(cmd),
            None => Err(JourneyError::Config("No command provided".to_string()))
        }
    }

    pub fn run_journeyctl_command(&mut self, command: crate::journeyctl::Commands) -> Result<(), JourneyError> {
        match command {
            crate::journeyctl::Commands::Init { path, name, vault_type: _, obsidian } => {
                self.init_vault(path, name, obsidian)
            }
            crate::journeyctl::Commands::List => {
                self.list_vaults()
            }
            crate::journeyctl::Commands::SetDefault { vault_name } => {
                self.set_default_vault(&vault_name)
            }
            crate::journeyctl::Commands::UnsetDefault => {
                self.unset_default_vault()
            }
            crate::journeyctl::Commands::ShowDefault => {
                self.show_default_vault()
            }
            crate::journeyctl::Commands::UnlistVault { vault_name } => {
                self.unlist_vault(&vault_name)
            }
            crate::journeyctl::Commands::Today { vault, verbose } => {
                self.show_today_file(vault, verbose)
            }
        }
    }

    fn handle_command_with_args(&mut self, cmd: crate::cli::Commands, vault: Option<String>, date: Option<String>, relative_date: Option<i64>, time: Option<String>, time_format: Option<String>) -> Result<(), JourneyError> {
        match cmd {
            crate::cli::Commands::Add { content } => {
                let cli_args = CliArgs { vault, date, relative_date, time, time_format };
                self.add_note(&content, &cli_args, None)
            }
            crate::cli::Commands::List => {
                let cli_args = CliArgs { vault, date, relative_date, time, time_format };
                self.list_notes(&cli_args, false, None)
            }
            crate::cli::Commands::Edit => {
                let cli_args = CliArgs { vault, date, relative_date, time, time_format };
                self.edit_notes(&cli_args)
            }
        }
    }

    fn handle_default_behavior(&mut self, cli: &Cli) -> Result<(), JourneyError> {
        let cli_args = CliArgs {
            vault: cli.vault.clone(),
            date: cli.date.clone(),
            relative_date: cli.relative_date,
            time: cli.time.clone(),
            time_format: cli.time_format.clone(),
        };
        
        if cli.list {
            self.list_notes(&cli_args, cli.header, cli.category.as_deref())
        } else if cli.edit {
            self.edit_notes(&cli_args)
        } else if cli.stdin {
            self.handle_stdin_input(&cli_args, cli.category.as_deref())
        } else if let Some(note) = &cli.add_note {
            self.add_note(note, &cli_args, cli.category.as_deref())
        } else if !cli.note_content.is_empty() {
            // Default behavior: treat note_content as note content
            let content = cli.note_content.join(" ");
            self.add_note(&content, &cli_args, cli.category.as_deref())
        } else {
            // Default behavior: list today's notes (same as --list)
            self.list_notes(&cli_args, cli.header, cli.category.as_deref())
        }
    }

    pub fn init_vault(&mut self, path: std::path::PathBuf, name: Option<String>, obsidian: bool) -> Result<(), JourneyError> {
        if obsidian {
            self.init_obsidian_vault(path, name)
        } else {
            self.init_regular_vault(path, name)
        }
    }

    fn init_regular_vault(&mut self, path: std::path::PathBuf, name: Option<String>) -> Result<(), JourneyError> {
        // Create vault directory
        std::fs::create_dir_all(&path)?;

        // Determine vault name - use provided name or path basename
        let vault_name = if let Some(name) = name {
            name
        } else {
            match path.file_name().and_then(|n| n.to_str()) {
                Some(s) if !s.is_empty() => s.to_string(),
                _ => return Err(JourneyError::Config("Invalid path: cannot extract basename".to_string())),
            }
        };

        // Get system locale
        let locale = self.get_system_locale();

        // Create vault config
        let vault_config = crate::config::VaultConfig {
            name: vault_name.clone(),
            path,
            locale,
            phrases: std::collections::HashMap::new(),
            section_header: None,
            section_header_work: None,
            section_header_personal: None,
            section_header_health: None,
            section_header_meetings: None,
            table_headers: None,
            date_format: None,
            template_file: None,
            file_path_format: None,
            list_type: None,
            section_name: None,
            weekly_format: None,
            monthly_format: None,
            quarterly_format: None,
            yearly_format: None,
            note_format: None,
        };

        // Add to config and save
        self.config.add_vault(vault_config);
        self.config_manager.save_config(&self.config)?;

        println!("Vault '{}' initialized successfully!", vault_name);
        Ok(())
    }

    fn init_obsidian_vault(&mut self, path: std::path::PathBuf, name: Option<String>) -> Result<(), JourneyError> {
        // Validate that the path exists and is an Obsidian vault
        if !path.exists() {
            return Err(JourneyError::Config(format!("Obsidian vault path does not exist: {}", path.display())));
        }

        // Check if it's a valid Obsidian vault by looking for .obsidian directory
        let obsidian_dir = path.join(".obsidian");
        if !obsidian_dir.exists() {
            return Err(JourneyError::Config(format!("Path is not a valid Obsidian vault (missing .obsidian directory): {}", path.display())));
        }

        // Determine vault name - use provided name or path basename
        let vault_name = if let Some(name) = name {
            name
        } else {
            path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| JourneyError::Config("Invalid path: cannot extract basename".to_string()))?
                .to_string()
        };

        // Check for required Obsidian plugins
        let plugin_configs = self.check_obsidian_plugins(&obsidian_dir)?;

        // Print plugin detection results
        println!("🔍 Detected Obsidian plugins:");
        if let Some(_) = &plugin_configs.daily_notes {
            println!("Daily Notes (core plugin) - enabled");
        } else {
            println!("Daily Notes (core plugin) - not enabled");
        }

        if let Some(_) = &plugin_configs.periodic_notes {
            println!("Periodic Notes plugin - enabled");
        } else {
            println!("Periodic Notes plugin - not enabled");
        }

        if !plugin_configs.journals.is_empty() {
            println!("Journals plugin - enabled ({} journal(s) configured)", plugin_configs.journals.len());
            for journal in &plugin_configs.journals {
                println!("Journal: '{}'", journal.name);
            }
        } else {
            println!("Journals plugin - not configured");
        }

        // Get system locale
        let locale = self.get_system_locale();

        // Create vault config with Obsidian-specific settings
        let mut vault_config = crate::config::VaultConfig {
            name: vault_name.clone(),
            path,
            locale,
            phrases: std::collections::HashMap::new(),
            section_header: None,
            section_header_work: None,
            section_header_personal: None,
            section_header_health: None,
            section_header_meetings: None,
            table_headers: None,
            date_format: None,
            template_file: None,
            file_path_format: None,
            list_type: None,
            section_name: None,
            weekly_format: None,
            monthly_format: None,
            quarterly_format: None,
            yearly_format: None,
            note_format: None,
        };

        // Apply Obsidian plugin configurations (excluding journals for now)
        self.apply_obsidian_config(&mut vault_config, &plugin_configs)?;

        // Create vaults for each journal
        let mut vault_count = 0;
        for journal in &plugin_configs.journals {
            let mut journal_vault_config = vault_config.clone();
            
            // Set journal-specific name
            journal_vault_config.name = format!("{}-{}", vault_name, journal.name);
            
            // Apply journal-specific configuration
            
            // Add journal vault to config
            self.config.add_vault(journal_vault_config);
            vault_count += 1;
        }

        // If no journals found, create a single vault with the base configuration
        if vault_count == 0 {
            self.config.add_vault(vault_config);
            vault_count = 1;
        }

        // Save configuration
        self.config_manager.save_config(&self.config)?;

        println!();
        println!("🎉 Obsidian vault '{}' initialized successfully with {} vault(s)!", vault_name, vault_count);
        
        // Print important configuration reminder
        println!();
        println!("   IMPORTANT: Please review your configuration and add the missing essential information:");
        println!("   • section_header: The default section header in the daily note where journey will put your notes (e.g., '## Todays notes')");
        println!("   • list_type: The format for your notes ('bullet' or 'table')");
        println!("   • file_path_format: template string journey uses to determine the location of the daily note");
        println!("   • It can contain variables like {{year}}, {{month}}, {{day}}, {{weekday}}, {{weekday_short}}, {{month_name}}, {{month_short}}, etc");
        println!("   • The variables are replaced with the actual values when the note is added");
        println!();
        println!();
        println!("   You can edit the configuration file at: {}", self.config_manager.config_path.display());
        println!();
        
        // Print configuration summary
        if let Some(daily_notes) = &plugin_configs.daily_notes {
            println!("Daily Notes: format='{}', folder='{}'", daily_notes.format, daily_notes.folder);
        }
        
        if let Some(periodic_notes) = &plugin_configs.periodic_notes {
            println!("Periodic Notes: enabled={}", periodic_notes.enabled);
        }
        
        if !plugin_configs.journals.is_empty() {
            println!("Journals: {} configured", plugin_configs.journals.len());
        }
        
        Ok(())
    }

    fn check_obsidian_plugins(&self, obsidian_dir: &std::path::Path) -> Result<ObsidianPluginConfigs, JourneyError> {
        let mut configs = ObsidianPluginConfigs::new();

        // Check for Daily Notes core plugin
        if let Ok(daily_notes_config) = self.check_daily_notes_plugin(obsidian_dir) {
            configs.daily_notes = Some(daily_notes_config);
        }

        // Check for Periodic Notes plugin
        if let Ok(periodic_notes_config) = self.check_periodic_notes_plugin(obsidian_dir) {
            configs.periodic_notes = Some(periodic_notes_config);
        }

        // Check for Journals plugin
        if let Ok(journals_configs) = self.check_journals_plugin(obsidian_dir) {
            configs.journals = journals_configs;
        }

        Ok(configs)
    }

    fn check_daily_notes_plugin(&self, obsidian_dir: &std::path::Path) -> Result<DailyNotesConfig, JourneyError> {
        let app_json_path = obsidian_dir.join("app.json");
        if !app_json_path.exists() {
            return Err(JourneyError::Config("Obsidian app.json not found".to_string()));
        }

        let app_json_content = std::fs::read_to_string(&app_json_path)?;
        let app_config: serde_json::Value = serde_json::from_str(&app_json_content)?;

        // Check if Daily Notes is enabled
        if let Some(daily_notes) = app_config.get("dailyNotes") {
            if let Some(enabled) = daily_notes.get("enabled") {
                if enabled.as_bool().unwrap_or(false) {
                    let format = daily_notes.get("format").and_then(|v| v.as_str()).unwrap_or("YYYY-MM-DD").to_string();
                    let folder = daily_notes.get("folder").and_then(|v| v.as_str()).unwrap_or("/").to_string();
                    let template = daily_notes.get("template").and_then(|v| v.as_str()).map(|s| s.to_string());

                    return Ok(DailyNotesConfig {
                        enabled: true,
                        format,
                        folder,
                        template,
                    });
                }
            } else {
                // If enabled field is missing, assume plugin is disabled
                return Err(JourneyError::Config("Daily Notes plugin not enabled".to_string()));
            }
        }

        Err(JourneyError::Config("Daily Notes plugin not enabled".to_string()))
    }

    fn check_periodic_notes_plugin(&self, obsidian_dir: &std::path::Path) -> Result<PeriodicNotesConfig, JourneyError> {
        let plugins_dir = obsidian_dir.join("plugins");
        let periodic_notes_dir = plugins_dir.join("periodic-notes");
        
        if !periodic_notes_dir.exists() {
            return Err(JourneyError::Config("Periodic Notes plugin not installed".to_string()));
        }

        let data_json_path = periodic_notes_dir.join("data.json");
        if !data_json_path.exists() {
            return Err(JourneyError::Config("Periodic Notes plugin data not found".to_string()));
        }

        let data_content = std::fs::read_to_string(&data_json_path)?;
        let plugin_data: serde_json::Value = serde_json::from_str(&data_content)?;

        // Check if plugin is enabled
        if let Some(enabled) = plugin_data.get("enabled") {
            if !enabled.as_bool().unwrap_or(false) {
                return Err(JourneyError::Config("Periodic Notes plugin not enabled".to_string()));
            }
        } else {
            // If enabled field is missing, assume plugin is disabled
            return Err(JourneyError::Config("Periodic Notes plugin not enabled".to_string()));
        }

        // Extract configuration
        Ok(PeriodicNotesConfig {
            enabled: true,
        })
    }

    fn check_journals_plugin(&self, obsidian_dir: &std::path::Path) -> Result<Vec<JournalsConfig>, JourneyError> {
        let plugins_dir = obsidian_dir.join("plugins");
        let journals_dir = plugins_dir.join("journals");
        
        if !journals_dir.exists() {
            return Err(JourneyError::Config("Journals plugin not installed".to_string()));
        }

        let data_json_path = journals_dir.join("data.json");
        if !data_json_path.exists() {
            return Err(JourneyError::Config("Journals plugin data not found".to_string()));
        }

        let data_content = std::fs::read_to_string(&data_json_path)?;
        let plugin_data: serde_json::Value = serde_json::from_str(&data_content)?;

        // Check if plugin is enabled (Journals plugin doesn't have a simple enabled field)
        // We'll consider it enabled if it has journals configured
        if !plugin_data.get("journals").is_some() {
            return Err(JourneyError::Config("Journals plugin not configured".to_string()));
        }

        // Extract configuration from all journal entries
        let journals = plugin_data.get("journals")
            .ok_or_else(|| JourneyError::Config("No journals configured".to_string()))?;
        
        let mut journal_configs = Vec::new();
        
        if let Some(journals_obj) = journals.as_object() {
            for (journal_name, _journal_data) in journals_obj {
                // Extract configuration from each journal entry
                journal_configs.push(JournalsConfig {
                    name: journal_name.clone(),
                });
            }
        } else {
            return Err(JourneyError::Config("Invalid journals configuration".to_string()));
        }

        if journal_configs.is_empty() {
            return Err(JourneyError::Config("No journal entries found".to_string()));
        }

        Ok(journal_configs)
    }

    fn apply_obsidian_config(&mut self, vault_config: &mut crate::config::VaultConfig, plugin_configs: &ObsidianPluginConfigs) -> Result<(), JourneyError> {
        // Apply Daily Notes configuration
        if let Some(daily_notes) = &plugin_configs.daily_notes {
            if daily_notes.enabled {
                vault_config.date_format = Some(daily_notes.format.clone());
                vault_config.file_path_format = Some(daily_notes.folder.clone());
                
                if let Some(template) = &daily_notes.template {
                    vault_config.template_file = Some(template.clone());
                }
            }
        }

        // Apply Periodic Notes configuration
        if let Some(periodic_notes) = &plugin_configs.periodic_notes {
            if periodic_notes.enabled {
                // Periodic notes configuration is enabled
            }
        }

        // Apply Journals configuration - this will be handled separately to create multiple vaults

        Ok(())
    }

    fn add_note(&mut self, content: &str, cli: &CliArgs, category: Option<&str>) -> Result<(), JourneyError> {
        let vault = self.get_vault(cli.vault.as_deref())?;
        let date = self.parse_date(cli)?;
        let time = self.parse_time(cli)?;
        
        let timestamp = if let Some(time) = time {
            vault.date_handler.combine_date_time(date, time)
        } else {
            // Use current time with the specified date (or current date if not specified)
            let current_time = vault.date_handler.get_current_datetime().time();
            vault.date_handler.combine_date_time(date, current_time)
        };

        vault.add_note_with_category(content, Some(timestamp), category)?;
        println!("Note added successfully!");
        Ok(())
    }

    fn list_notes(&self, cli: &CliArgs, header: bool, category: Option<&str>) -> Result<(), JourneyError> {
        let vault = self.get_vault(cli.vault.as_deref())?;
        let date = self.parse_date(cli)?;
        
        let notes = vault.list_notes_with_category(date, category)?;
        
        if notes.is_empty() {
            println!("No notes found for {}", vault.date_handler.format_date(date));
        } else {
            let has_table_format = notes.iter().any(|note| note.trim().starts_with("|"));

            // Suppress "Notes for" message when in table mode
            if !header && !has_table_format {
                println!("Notes for {}:", vault.date_handler.format_date(date));
            }

            // If header flag is set and we have table format notes, include table headers
            if header && has_table_format {
                let (time_header, content_header) = vault.get_table_headers();
                println!("| {} | {} |", time_header, content_header);
                println!("|------|----------|");
            }
            
            for note in notes {
                println!("{}", note);
            }
        }
        
        Ok(())
    }

    fn edit_notes(&self, cli: &CliArgs) -> Result<(), JourneyError> {
        let vault = self.get_vault(cli.vault.as_deref())?;
        let date = self.parse_date(cli)?;
        let note_path = vault.get_editor_path(date);

        // Get editor from environment
        let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
        
        // Open editor
        let status = Command::new(&editor)
            .arg(&note_path)
            .status()
            .map_err(|_| JourneyError::EditorNotFound(editor))?;

        if !status.success() {
            eprintln!("Editor exited with non-zero status");
        }

        Ok(())
    }


    pub fn get_vault(&self, vault_name: Option<&str>) -> Result<Vault, JourneyError> {
        // Explicitly error if no vaults configured
        if self.config.vaults.is_empty() {
            return Err(JourneyError::VaultNotFound("No vaults configured. Use 'journey init' to create one.".to_string()));
        }

        let vault_config = if let Some(name) = vault_name {
            // User explicitly specified a vault
            self.config.get_vault(name)
                .ok_or_else(|| JourneyError::VaultNotFound(name.to_string()))?
        } else {
            // No vault specified - use default vault or smart selection
            if let Some(default_name) = &self.config.default_vault {
                // Use the explicitly set default vault
                self.config.get_vault(default_name)
                    .ok_or_else(|| JourneyError::VaultNotFound(format!("Default vault '{}' not found", default_name)))?
            } else if self.config.vaults.len() == 1 {
                // Only one vault exists - use it automatically
                self.config.vaults.values().next()
                    .ok_or_else(|| JourneyError::VaultNotFound("No vaults configured".to_string()))?
            } else if self.config.vaults.len() > 1 {
                // Multiple vaults exist - need to specify which one or set a default
                return Err(JourneyError::VaultNotFound(
                    format!("Multiple vaults available: {}. Please specify --vault or set a default vault", 
                        self.config.vaults.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                ));
            } else {
                // No vaults configured
                return Err(JourneyError::VaultNotFound("No vaults configured. Use 'journey init' to create one.".to_string()));
            }
        };

        Ok(Vault::new(vault_config.clone()))
    }

    pub fn parse_date(&self, cli: &CliArgs) -> Result<NaiveDate, JourneyError> {
        if let Some(date_str) = &cli.date {
            let vault = self.get_vault(cli.vault.as_deref())?;
            vault.date_handler.parse_date_with_format_override(date_str, vault.config.date_format.as_deref())
        } else if let Some(days_ago) = cli.relative_date {
            let vault = self.get_vault(cli.vault.as_deref())?;
            Ok(vault.date_handler.parse_relative_date(days_ago))
        } else {
            Ok(Local::now().date_naive())
        }
    }

    fn parse_time(&self, cli: &CliArgs) -> Result<Option<chrono::NaiveTime>, JourneyError> {
        if let Some(time_str) = &cli.time {
            let vault = self.get_vault(cli.vault.as_deref())?;
            Ok(Some(vault.date_handler.parse_time_with_format_override(time_str, cli.time_format.as_deref())?))
        } else {
            Ok(None)
        }
    }

    fn handle_stdin_input(&mut self, cli: &CliArgs, category: Option<&str>) -> Result<(), JourneyError> {
        use std::io::{self, BufRead};
        
        let stdin = io::stdin();
        let lines = stdin.lock().lines();
        
        let mut note_count = 0;
        
        for line in lines {
            let line = line?;
            let trimmed = line.trim();
            
            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }
            
            // Add each line as a separate note
            self.add_note(trimmed, cli, category)?;
            note_count += 1;
        }
        
        if note_count > 0 {
            println!("Added {} notes from stdin", note_count);
        } else {
            println!("No content received from stdin");
        }
        
        Ok(())
    }

    fn get_system_locale(&self) -> String {
        // Try to get locale from environment
        env::var("LANG")
            .or_else(|_| env::var("LC_ALL"))
            .unwrap_or_else(|_| "en-US".to_string())
    }

    fn set_default_vault(&mut self, vault_name: &str) -> Result<(), JourneyError> {
        match self.config.set_default_vault(vault_name) {
            Ok(()) => {
                self.config_manager.save_config(&self.config)?;
                println!("Default vault set to '{}'", vault_name);
                Ok(())
            }
            Err(error) => Err(JourneyError::Config(error))
        }
    }

    fn unset_default_vault(&mut self) -> Result<(), JourneyError> {
        self.config.unset_default_vault();
        self.config_manager.save_config(&self.config)?;
        println!("Default vault unset");
        Ok(())
    }

    fn show_default_vault(&self) -> Result<(), JourneyError> {
        if let Some(default_name) = &self.config.default_vault {
            if let Some(vault) = self.config.get_vault(default_name) {
                println!("Default vault: {} ({})", default_name, vault.path.display());
            } else {
                println!("Default vault '{}' is set but vault not found", default_name);
            }
        } else {
            println!("No default vault set");
            if self.config.vaults.len() > 1 {
                println!("Available vaults: {}", 
                    self.config.vaults.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
            }
        }
        Ok(())
    }

    pub fn unlist_vault(&mut self, vault_name: &str) -> Result<(), JourneyError> {
        // Check if vault exists
        if !self.config.vaults.contains_key(vault_name) {
            return Err(JourneyError::VaultNotFound(format!("Vault '{}' not found", vault_name)));
        }

        // Remove vault from config
        self.config.remove_vault(vault_name)
            .map_err(|e| JourneyError::Config(e))?;

        // Save updated config
        self.config_manager.save_config(&self.config)?;

        println!("Vault '{}' unlisted successfully!", vault_name);
        Ok(())
    }

    // Test helper methods
    #[doc(hidden)]
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    fn list_vaults(&self) -> Result<(), JourneyError> {
        if self.config.vaults.is_empty() {
            println!("No vaults configured.");
            println!("Use 'journeyctl init --path <path> --name <name>' to create your first vault.");
            return Ok(());
        }

        println!("Configured vaults:");
        println!();

        for (name, vault) in &self.config.vaults {
            let is_default = self.config.default_vault.as_ref() == Some(name);
            let default_marker = if is_default { " (default)" } else { "" };
            
            println!("  {}: {}{}", name, vault.path.display(), default_marker);
            
            // Show additional vault information
            if let Some(section_header) = &vault.section_header {
                println!("    Section: {}", section_header);
            }
            if let Some(template_file) = &vault.template_file {
                println!("    Template: {}", template_file);
            }
            if let Some(file_path_format) = &vault.file_path_format {
                println!("    Path format: {}", file_path_format);
            }
            if !vault.phrases.is_empty() {
                println!("    Phrases: {} configured", vault.phrases.len());
            }
            println!();
        }

        if let Some(default_name) = &self.config.default_vault {
            println!("Default vault: {}", default_name);
        } else {
            println!("No default vault set");
        }

        Ok(())
    }

    fn show_today_file(&self, vault_name: Option<String>, verbose: bool) -> Result<(), JourneyError> {
        let vault = self.get_vault(vault_name.as_deref())?;
        let today = vault.date_handler.get_current_datetime().date_naive();
        let file_path = vault.get_note_path(today);
        
        if verbose {
            println!("Today's file location: {}", file_path.display());
            
            if file_path.exists() {
                println!("File exists: Yes");
            } else {
                println!("File exists: No");
            }
        } else {
            println!("{}", file_path.display());
        }
        
        Ok(())
    }
}
