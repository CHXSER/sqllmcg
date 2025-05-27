use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub sonar_host: String,
    pub ollama_url: String,
    pub model: String,
    pub token: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = "config.json";
        
        if Path::new(config_path).exists() {
            let contents = fs::read_to_string(config_path)
                .context("Failed to read config file")?;
            serde_json::from_str(&contents)
                .context("Failed to parse config file")
        } else {
            let config = Config {
                sonar_host: "http://localhost:9000".to_string(),
                ollama_url: "http://padova.zucchettitest.it:11434".to_string(),
                model: "deepseek-r1:14b".to_string(),
                token: "token".to_string(),
            };
            
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = "config.json";
        let contents = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(config_path, contents)
            .context("Failed to write config file")
    }

    pub fn update(&mut self, sonar_host: Option<String>, ollama_url: Option<String>, token: Option<String>, model: Option<String>) -> Result<()> {
        if let Some(host) = sonar_host {
            self.sonar_host = host;
        }
        if let Some(url) = ollama_url {
            self.ollama_url = url;
        }
        if let Some(t) = token {
            self.token = t;
        }
        if let Some(m) = model {
            self.model = m;
        }
        self.save()
    }
}