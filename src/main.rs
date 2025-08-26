mod api; //imports the api features 
mod spinner;
mod imageAnalyzer;
mod clipboard;
use anyhow::{Context, Result};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // imageAnalyzer::ai_request().await?; 
     clipboard::ai_request().await?;
    Ok(())
}

