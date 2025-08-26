mod api; //imports the api features 
mod spinner;
mod imageAnalyzer;
use anyhow::{Context, Result};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    imageAnalyzer::imageAnalyzer().await?; 
    Ok(())
}

