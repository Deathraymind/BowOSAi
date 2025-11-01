use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs;

#[derive(Deserialize)]
struct Secrets {
    #[serde(rename = "OPENAI_API_KEY")]
    openai_api_key: String,
}

/// Return the path where the OpenAI secrets file should live.
pub fn config_path() -> PathBuf {
    if let Some(dir) = dirs::config_dir() {
        dir.join("openai").join("openai.json")
    } else {
        PathBuf::from(std::env::var("HOME").expect("HOME not set"))
            .join(".config/openai/openai.json")
    }
}

/// Load the API key from JSON file. If the file is missing, tell the user what to do.
pub async fn load_api_key() -> Result<String> {
    let path = config_path();

    // Check existence first
    if !fs::try_exists(&path).await? {
        return Err(anyhow::anyhow!(
            "Config file not found: {}\nPlease create this file and populate it with your API key in JSON format, e.g.:\n\n{{ \"OPENAI_API_KEY\": \"sk-...\" }}",
            path.display()
        ));
    }

    let bytes = fs::read(&path)
        .await
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let secrets: Secrets = serde_json::from_slice(&bytes)
        .with_context(|| format!("Invalid JSON in {}", path.display()))?;

    Ok(secrets.openai_api_key)
}

// // 1) Load API key from ~/.config/openai/openai.json
//    let api_key = api::load_api_key().await?; // api::load... will load from api.rs
//    let cfg = OpenAIConfig::new().with_api_key(api_key);
//    let client = Client::with_config(cfg);

