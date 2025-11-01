use anyhow::{Result, Context};
use async_openai::{Client, config::OpenAIConfig};
use base64::Engine;
use serde_json::json;
use tokio::fs;

use crate::api;

pub async fn ai_request(file_path: &str) -> Result<String> {
    let api_key = api::load_api_key().await?;
    let cfg = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(cfg);

    let bytes = fs::read(file_path)
        .await
        .with_context(|| format!("Failed to read image at {}", file_path))?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:image/png;base64,{}", b64);

    let req = json!({
        "model": "gpt-4o-mini",
        "messages": [{
          "role": "user",
          "content": [
            { "type": "text", "text": "if the image is a question please give the correct answer and a brief explanation: if its of a page summerize it or anything else really" },
            { "type": "image_url", "image_url": { "url": data_url } }
          ]
        }]
    });

    let resp: serde_json::Value = client.chat().create_byot(req).await?;

    let out = resp["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string();


    Ok(out)
}

