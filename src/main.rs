use anyhow::{Context, Result};
use async_openai::{config::OpenAIConfig, Client};
use base64::Engine as _; // for .encode()
use serde::Deserialize;
use serde_json::json;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::fs;

#[derive(Deserialize)]
struct Secrets {
    #[serde(rename = "OPENAI_API_KEY")]
    openai_api_key: String,
}

fn config_path() -> PathBuf {
    // ~/.config/openai/openai.json (respects XDG_CONFIG_HOME) some os bs tbh
    if let Some(dir) = dirs::config_dir() {
        dir.join("openai").join("openai.json")
    } else {
        // Fallback idk this seems to fix issue like half the time
        PathBuf::from(std::env::var("HOME").expect("HOME not set"))
            .join(".config/openai/openai.json")
    }
}

async fn load_api_key() -> Result<String> {
    let path = config_path();
    let bytes = fs::read(&path)
        .await
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let secrets: Secrets = serde_json::from_slice(&bytes)
        .with_context(|| format!("Invalid JSON in {}", path.display()))?;
    Ok(secrets.openai_api_key)
}

/// Run any future while showing a spinner on stderr.
/// The spinner clears itself before returning.
async fn with_spinner<F, T>(msg: &'static str, fut: F) -> T
where
    F: std::future::Future<Output = T>,
{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // let frames = ["   ", ".  ", ".. ", "...", " ..", "  ."];
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    let handle = tokio::spawn(async move {
        let mut i = 0usize;
        while r.load(Ordering::Relaxed) {
            eprint!("\r{} {}", frames[i % frames.len()], msg);
            let _ = io::stderr().flush();
            i = i.wrapping_add(1);
            tokio::time::sleep(Duration::from_millis(80)).await;
        }
        // Clear the line
        eprint!("\r\x1b[2K");
        let _ = io::stderr().flush();
    });

    let out = fut.await;
    running.store(false, Ordering::Relaxed);
    let _ = handle.await;
    out
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1) Load API key from ~/.config/openai/openai.json
    let api_key = load_api_key().await?;
    let cfg = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(cfg);

    // 2) Read image
    let file_path = "/home/bowyn/Pictures/screenshots/temp_screenshot.png";
    let bytes = fs::read(file_path)
        .await
        .with_context(|| format!("Failed to read image at {}", file_path))?;

    // 3) Base64 → data URL
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:image/png;base64,{}", b64);

    // 4) BYOT/vision-style payload
    let req = json!({
        "model": "gpt-5-mini",
        "messages": [{
          "role": "user",
          "content": [
            { "type": "text", "text": "please give the correct answer and a brief explanation:" },
            { "type": "image_url", "image_url": { "url": data_url } }
          ]
        }]
    });

    // 5) Send & print, wrapped with spinner
    let resp: serde_json::Value =
        with_spinner("thinking…", client.chat().create_byot(req)).await?;
    let out = resp["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default();

    // Make sure the next print starts on a fresh line (spinner cleared already)
    println!("{out}");
    Ok(())
}

