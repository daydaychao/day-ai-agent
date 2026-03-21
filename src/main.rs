mod config;

use clap::{Parser, Subcommand};
use dialoguer::Select;
use reqwest::Client;
use serde_json::json;

#[derive(Parser)]
#[command(name = "dayai")]
#[command(version = "0.1.0")]
#[command(about = "Cloud Lobster AI Agent CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 設定：API Key 和模型
    Setup,
    /// 主要邏輯：呼叫 Gemini API
    Main {
        /// 自定義 prompt（可選）
        #[arg(short, long)]
        prompt: Option<String>,
    },
    /// 更新 dayai 到最新版本
    Update {
        /// 指定版本（可選）
        #[arg(short, long)]
        version: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup {} => {
            run_setup().await?;
        }
        Commands::Main { prompt } => {
            run_main(prompt).await?;
        }
        Commands::Update { version } => {
            run_update(version).await?;
        }
    }

    Ok(())
}

fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    match config::load_config() {
        Ok(cfg) => {
            if !cfg.api.key.is_empty() {
                return Ok(cfg.api.key);
            }
        }
        Err(_) => {}
    }

    Err("錯誤：找不到 GEMINI_API_KEY。請執行 'dayai setup' 設定，或設定環境變數 GEMINI_API_KEY。".into())
}

const LIGHT_GREEN: &str = "\x1b[92m";
const RESET: &str = "\x1b[0m";

async fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    let items = &[
        "設定 GEMINI_API_KEY",
        "選擇預設模型",
    ];

    let selections = &[
        config::has_api_key(),
        config::has_model(),
    ];

    let formatted_items: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let checked = if selections[i] { "[✓]" } else { "[ ]" };
            format!("{}{} {}{}", LIGHT_GREEN, checked, item, RESET)
        })
        .collect();

    let selection = Select::new()
        .with_prompt("請選擇操作")
        .default(0)
        .clear(true)
        .items(&formatted_items)
        .interact_opt()?
        .ok_or("已取消")?;

    match selection {
        0 => {
            let key = config::prompt_for_key()?;
            config::save_config(&key)?;
        }
        1 => {
            let api_key = get_api_key()?;
            let models = config::fetch_models(&api_key).await?;

            let formatted_models: Vec<String> = models
                .iter()
                .map(|m| format!("{}{}{}", LIGHT_GREEN, m, RESET))
                .collect();

            let selected = Select::new()
                .with_prompt("請選擇模型")
                .default(0)
                .clear(true)
                .items(&formatted_models)
                .interact_opt()?
                .ok_or("已取消")?;

            config::save_model(&models[selected])?;
            println!("✅ 已選擇：{}", models[selected]);
        }
        _ => {}
    }

    Ok(())
}

async fn run_main(prompt: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_api_key()?;
    let model = config::get_model();

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        model
    );

    let user_prompt = prompt.unwrap_or_else(|| {
        "你是一位資深獵頭，請用 JSON 格式提供一個虛構的日本遠端前端職缺。請確保輸出僅包含 JSON 內容。".to_string()
    });

    let response = client
        .post(&url)
        .query(&[("key", &api_key)])
        .header("Content-Type", "application/json")
        .json(&json!({
            "contents": [{
                "parts": [{
                    "text": user_prompt
                }]
            }]
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let err_text = response.text().await?;
        eprintln!("API 請求失敗，狀態碼: {}\n內容: {}", status, err_text);
        return Ok(());
    }

    let text_result = response.text().await?;
    println!("{}", text_result);

    Ok(())
}

async fn run_update(_version: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("正在檢查更新...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("daydaychao")
        .repo_name("day-ai-agent")
        .bin_name("dayai")
        .show_output(false)
        .show_download_progress(true)
        .build()?
        .update()?;

    println!("更新成功！新版本: {}", status.version());

    Ok(())
}
