mod api; //imports the api features 
mod spinner;
use anyhow::{Context, Result};
use async_openai::{config::OpenAIConfig, Client};
use base64::Engine as _; // for .encode()
use serde_json::json;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // 1) Load API key from ~/.config/openai/openai.json
    let api_key = api::load_api_key().await?; // api::load... will load from api.rs
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
            { "type": "text", "text": "if the image is a question please give the correct answer and a brief explanation: if its of a page summerize it or anything else really" },
            { "type": "image_url", "image_url": { "url": data_url } }
          ]
        }]
    });

    // 5) Send & print, wrapped with spinner
    let resp: serde_json::Value =
        spinner::with_spinner("thinking…", client.chat().create_byot(req)).await?;
    let out = resp["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default();

    // Make sure the next print starts on a fresh line (spinner cleared already)
    println!("{out}");
    Ok(())
}

