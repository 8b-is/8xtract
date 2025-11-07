use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub ocr: OcrConfig,
    
    #[serde(default)]
    pub extraction: ExtractionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    #[serde(default = "default_api_endpoint")]
    pub api_endpoint: String,
    
    #[serde(default = "default_model")]
    pub model: String,
    
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfig {
    #[serde(default = "default_output_format")]
    pub output_format: String,
    
    #[serde(default = "default_preserve_layout")]
    pub preserve_layout: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ocr: OcrConfig::default(),
            extraction: ExtractionConfig::default(),
        }
    }
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            api_endpoint: default_api_endpoint(),
            model: default_model(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
        }
    }
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            output_format: default_output_format(),
            preserve_layout: default_preserve_layout(),
        }
    }
}

fn default_api_endpoint() -> String {
    "http://localhost:8000/v1".to_string()
}

fn default_model() -> String {
    "deepseek-ocr".to_string()
}

fn default_max_tokens() -> usize {
    512
}

fn default_temperature() -> f32 {
    0.0
}

fn default_output_format() -> String {
    "markdown".to_string()
}

fn default_preserve_layout() -> bool {
    true
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            let config: Config = toml::from_str(&contents)
                .context("Failed to parse config file")?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }
        
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&config_path, contents)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?;
        // Use 'xtract' as directory name for better filesystem compatibility
        Ok(config_dir.join("xtract").join("config.toml"))
    }
}
