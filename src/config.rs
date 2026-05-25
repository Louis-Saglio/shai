use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub api_key: String,
}

pub enum ConfigError {
    Path(String),
    Read(String),
    Write(String),
    Parse(String),
    Serialize(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(e) => write!(f, "Path error: {}", e),
            Self::Read(e) => write!(f, "Read error: {}", e),
            Self::Write(e) => write!(f, "Write error: {}", e),
            Self::Parse(e) => write!(f, "Parse error: {}", e),
            Self::Serialize(e) => write!(f, "Serialize error: {}", e),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            return Ok(Config::default());
        }
        let content = fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::Read(format!("{}: {}", config_path.display(), e)))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::Parse(format!("{}: {}", config_path.display(), e)))?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::Path(format!("{}: {}", parent.display(), e)))?;
        }
        let content = toml::to_string(self).map_err(|e| ConfigError::Serialize(e.to_string()))?;
        fs::write(&config_path, content)
            .map_err(|e| ConfigError::Write(format!("{}: {}", config_path.display(), e)))?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf, ConfigError> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| ConfigError::Path("Could not find config directory".to_string()))?;
        path.push("shai");
        path.push("config.toml");
        Ok(path)
    }
}
