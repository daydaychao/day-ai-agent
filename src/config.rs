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
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig {
                key: String::new(),
            },
        }
    }
}

fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("找不到 home 目錄");
    home.join(".dayai").join("config.toml")
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();

    if !config_path.exists() {
        return Err("設定檔不存在，請先執行 dayai setkey".into());
    }

    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();

    // 確保目錄存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config = Config {
        api: ApiConfig {
            key: key.to_string(),
        },
    };

    let content = toml::to_string_pretty(&config)?;
    fs::write(&config_path, content)?;

    println!("設定成功！");
    Ok(())
}

pub fn prompt_for_key() -> Result<String, Box<dyn std::error::Error>> {
    print!("請輸入 GEMINI_API_KEY: ");
    io::stdout().flush()?;

    let key = rpassword::read_password()?;

    if key.trim().is_empty() {
        return Err("API Key 不能為空".into());
    }

    Ok(key)
}
