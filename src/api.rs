use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs;

#[derive(Deserialize)]
struct Secrets {
    #[serde(rename = "OPENAI_API_KEY")]
    openai_api_key: String,
}

/// Expose only what others need.
pub fn config_path() -> PathBuf {
    if let Some(dir) = dirs::config_dir() {
        dir.join("openai").join("openai.json")
    } else {
        PathBuf::from(std::env::var("HOME").expect("HOME not set"))
            .join(".config/openai/openai.json")
    }
}

pub async fn load_api_key() -> Result<String> {
    let path = config_path();
    let bytes = fs::read(&path)
        .await
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let secrets: Secrets = serde_json::from_slice(&bytes)
        .with_context(|| format!("Invalid JSON in {}", path.display()))?;
    Ok(secrets.openai_api_key)
}

