use crate::config::Config;
use crate::errors::JourneyError;
use dirs::config_dir;
use std::fs;
use std::path::PathBuf;
use std::env;

pub struct ConfigManager {
    pub config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, JourneyError> {
        // Check if a custom config path is specified via environment variable
        let config_path = if let Ok(custom_path) = env::var("JOURNEY_CONFIG") {
            PathBuf::from(custom_path)
        } else {
            let config_dir = config_dir()
                .ok_or_else(|| JourneyError::Config("Could not find config directory".to_string()))?;
            
            let journey_config_dir = config_dir.join("journey");
            fs::create_dir_all(&journey_config_dir)?;
            
            journey_config_dir.join("journey.yaml")
        };
        
        Ok(Self { config_path })
    }

    pub fn load_config(&self) -> Result<Config, JourneyError> {
        if !self.config_path.exists() {
            return Ok(Config::new());
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: Config = serde_yaml_ng::from_str(&content)?;
        Ok(config)
    }

    pub fn save_config(&self, config: &Config) -> Result<(), JourneyError> {
        let content = serde_yaml_ng::to_string(config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn config_exists(&self) -> bool {
        self.config_path.exists()
    }
}

