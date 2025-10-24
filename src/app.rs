use crate::cli::Cli;
use crate::config::Config;
use crate::config_manager::ConfigManager;
use crate::errors::JourneyError;
use crate::vault::Vault;
use chrono::{Local, NaiveDate};
use std::env;
use std::process::Command;

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
        
        // Check if config file exists
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

    fn handle_command_with_args(&mut self, cmd: crate::cli::Commands, vault: Option<String>, date: Option<String>, relative_date: Option<i64>, time: Option<String>, time_format: Option<String>) -> Result<(), JourneyError> {
        match cmd {
            crate::cli::Commands::Init { path, name, vault_type: _ } => {
                self.init_vault(path, name)
            }
            crate::cli::Commands::Add { content } => {
                let cli_args = CliArgs { vault, date, relative_date, time, time_format };
                self.add_note(&content, &cli_args)
            }
            crate::cli::Commands::List => {
                let cli_args = CliArgs { vault, date, relative_date, time, time_format };
                self.list_notes(&cli_args)
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
            self.list_notes(&cli_args)
        } else if cli.edit {
            self.edit_notes(&cli_args)
        } else if cli.stdin {
            self.handle_stdin_input(&cli_args)
        } else if let Some(note) = &cli.add_note {
            self.add_note(note, &cli_args)
        } else if !cli.note_content.is_empty() {
            // Default behavior: treat note_content as note content
            let content = cli.note_content.join(" ");
            self.add_note(&content, &cli_args)
        } else {
            // Default behavior: list today's notes (same as --list)
            self.list_notes(&cli_args)
        }
    }

    pub fn init_vault(&mut self, path: std::path::PathBuf, name: Option<String>) -> Result<(), JourneyError> {
        // Create vault directory
        std::fs::create_dir_all(&path)?;

        // Determine vault name - use provided name or path basename
        let vault_name = if let Some(name) = name {
            name
        } else {
            path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| JourneyError::Config("Invalid path: cannot extract basename".to_string()))?
                .to_string()
        };

        // Get system locale
        let locale = self.get_system_locale();

        // Create vault config
        let vault_config = crate::config::VaultConfig {
            name: vault_name.clone(),
            path,
            locale,
            phrases: std::collections::HashMap::new(),
            section_name: None,
            date_format: None,
            template_file: None,
            file_path_format: None,
        };

        // Add to config and save
        self.config.add_vault(vault_config);
        self.config_manager.save_config(&self.config)?;

        println!("Vault '{}' initialized successfully!", vault_name);
        Ok(())
    }

    fn add_note(&mut self, content: &str, cli: &CliArgs) -> Result<(), JourneyError> {
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

        vault.add_note(content, Some(timestamp))?;
        println!("Note added successfully!");
        Ok(())
    }

    fn list_notes(&self, cli: &CliArgs) -> Result<(), JourneyError> {
        let vault = self.get_vault(cli.vault.as_deref())?;
        let date = self.parse_date(cli)?;
        
        let notes = vault.list_notes(date)?;
        
        if notes.is_empty() {
            println!("No notes found for {}", vault.date_handler.format_date(date));
        } else {
            println!("Notes for {}:", vault.date_handler.format_date(date));
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
        let vault_config = if let Some(name) = vault_name {
            // User explicitly specified a vault
            self.config.get_vault(name)
                .ok_or_else(|| JourneyError::VaultNotFound(name.to_string()))?
        } else {
            // No vault specified - use smart selection
            if self.config.vaults.len() == 1 {
                // Only one vault exists - use it automatically
                self.config.vaults.values().next()
                    .ok_or_else(|| JourneyError::VaultNotFound("No vaults configured".to_string()))?
            } else if self.config.vaults.len() > 1 {
                // Multiple vaults exist - need to specify which one
                return Err(JourneyError::VaultNotFound(
                    format!("Multiple vaults available: {}. Please specify --vault", 
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

    fn handle_stdin_input(&mut self, cli: &CliArgs) -> Result<(), JourneyError> {
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
            self.add_note(trimmed, cli)?;
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
}
