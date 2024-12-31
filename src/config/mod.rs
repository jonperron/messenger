use serde::Deserialize;
use std::fs;
use std::path::Path;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub environment: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct TemplatesConfig {
    pub path: String,
    pub default_language: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub service: ServiceConfig,
    pub templates: TemplatesConfig,
    pub mailgun: MailgunConfig,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MailgunConfig {
    pub domain: String,
    pub api_key: String,
    pub base_url: Option<String>,
}