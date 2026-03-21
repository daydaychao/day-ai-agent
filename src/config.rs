use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub key: String,
    #[serde(default = "default_model")]
    pub model: String,
}

fn default_model() -> String {
    "gemini-2.0-flash-latest".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig {
                key: String::new(),
                model: default_model(),
            },
        }
    }
}

fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("Cannot find home directory");
    home.join(".dayai").join("config.toml")
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();

    if !config_path.exists() {
        return Err("Config file not found. Run 'dayai setup' first.".into());
    }

    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn load_config_or_default() -> Config {
    load_config().unwrap_or_default()
}

pub fn has_api_key() -> bool {
    match load_config() {
        Ok(cfg) => !cfg.api.key.trim().is_empty(),
        Err(_) => false,
    }
}

pub fn has_model() -> bool {
    match load_config() {
        Ok(cfg) => !cfg.api.model.trim().is_empty(),
        Err(_) => false,
    }
}

pub fn save_config(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut config = load_config_or_default();
    config.api.key = key.to_string();

    let content = toml::to_string_pretty(&config)?;
    fs::write(&config_path, content)?;

    println!("Configuration saved!");
    Ok(())
}

pub fn save_model(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut config = load_config_or_default();
    config.api.model = model.to_string();

    let content = toml::to_string_pretty(&config)?;
    fs::write(&config_path, content)?;

    println!("Model configuration saved!");
    Ok(())
}

pub fn prompt_for_key() -> Result<String, Box<dyn std::error::Error>> {
    print!("Enter GEMINI_API_KEY: ");
    io::stdout().flush()?;

    let key = rpassword::read_password()?;

    if key.trim().is_empty() {
        return Err("API Key cannot be empty".into());
    }

    Ok(key)
}

pub fn get_model() -> String {
    load_config()
        .map(|c| c.api.model)
        .unwrap_or_else(|_| default_model())
}

pub async fn fetch_models(api_key: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models?key={}",
        api_key
    );

    let response = client
        .get(&url)
        .send()
        .await?
        .text()
        .await?;

    #[derive(Deserialize)]
    struct Model {
        name: String,
    }

    #[derive(Deserialize)]
    struct ModelsResponse {
        models: Vec<Model>,
    }

    let parsed: ModelsResponse = serde_json::from_str(&response)?;

    let model_names: Vec<String> = parsed
        .models
        .into_iter()
        .map(|m| m.name.replace("models/", "").to_string())
        .collect();

    Ok(model_names)
}

use reqwest::Client;
