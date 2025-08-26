use crate::api; // imports the api features
use crate::spinner;
use anyhow::{Context, Result};
use async_openai::{config::OpenAIConfig, Client};
use serde_json::json;
use std::process::Command;

fn read_clipboard() -> Result<String> {
    let out = Command::new("wl-paste").arg("-n").output()?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        anyhow::bail!("wl-paste failed: {err}");
    }
    Ok(String::from_utf8(out.stdout)?)
}

pub async fn ai_request() -> Result<()> {
    // 1) Load API key
    let api_key = api::load_api_key().await?;
    let cfg = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(cfg);

    // 2) Grab clipboard text
    let clip = read_clipboard().context("reading clipboard")?;

    // 3) BYOT payload (text-only)
    let req = json!({
        "model": "gpt-5-mini",
        "messages": [{
          "role": "user",
          "content": [
            { "type": "text", "text":
              "If it's a question, answer and briefly explain; if it's a page, summarize it." },
            { "type": "text", "text": clip } // <-- use the variable directly
          ]
        }],
        "stream": false
    });

    // 4) Send & print
    let resp: serde_json::Value =
        spinner::with_spinner("thinking…", client.chat().create_byot(req)).await?;

    let out = resp["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default();
    println!("{out}");
    Ok(())
}

