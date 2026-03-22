mod agent;
mod config;

use clap::{Parser, Subcommand};
use dialoguer::{Select, theme::ColorfulTheme};

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
    /// Setup: API Key and model configuration
    Setup,
    /// Execute agent: call Gemini API to generate job listings
    Agent {
        /// Custom prompt (optional)
        #[arg(short, long)]
        prompt: Option<String>,
    },
    /// Update dayai to latest version
    Update {
        /// Specific version (optional)
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
        Commands::Agent { prompt } => {
            agent::run_agent(prompt).await?;
        }
        Commands::Update { version } => {
            run_update(version).await?;
        }
    }

    Ok(())
}

pub fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
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

    Err("Error: GEMINI_API_KEY not found. Run 'dayai setup' to configure, or set GEMINI_API_KEY environment variable.".into())
}

async fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    let current_model = config::get_model();
    let items = &[
        "Set GEMINI_API_KEY",
        &format!("Select default model  (current: {})", current_model),
    ];

    let selections = &[
        config::has_api_key(),
        config::has_model(),
    ];

    let formatted_items: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if selections[i] {
                format!("[✓] {}", item)
            } else {
                format!("[ ] {}", item)
            }
        })
        .collect();

    let mut theme = ColorfulTheme::default();
    theme.active_item_prefix = dialoguer::console::style(">>".to_string()).green().into();
    theme.inactive_item_prefix = dialoguer::console::style("   ".to_string()).into();

    let selection = Select::with_theme(&theme)
        .with_prompt("Select an option")
        .default(0)
        .clear(true)
        .items(&formatted_items)
        .interact_opt()?
        .ok_or("Cancelled")?;

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
                .map(|m| m.clone())
                .collect();

            let mut theme = ColorfulTheme::default();
            theme.active_item_prefix = dialoguer::console::style(">>".to_string()).green().into();
            theme.inactive_item_prefix = dialoguer::console::style("   ".to_string()).into();

            let selected = Select::with_theme(&theme)
                .with_prompt("Select a model")
                .default(0)
                .clear(true)
                .items(&formatted_models)
                .interact_opt()?
                .ok_or("Cancelled")?;

            config::save_model(&models[selected])?;
            println!("✅ Selected: {}", models[selected]);
        }
        _ => {}
    }

    Ok(())
}

async fn run_update(_version: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking for updates...");

    let current_version = env!("CARGO_PKG_VERSION");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("daydaychao")
        .repo_name("day-ai-agent")
        .bin_name("dayai")
        .current_version(current_version)
        .show_output(false)
        .show_download_progress(true)
        .build()?
        .update()?;

    println!("Update successful! New version: {}", status.version());

    Ok(())
}
