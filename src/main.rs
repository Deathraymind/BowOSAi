mod api; //imports the api features 
mod spinner;
mod imageAnalyzer;
mod clipboard;

use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use clipboard input
    #[arg(short = 'c', long = "clipboard")]
    c: bool,

    /// Use picture input
    #[arg(short = 'p', long = "picture")]
    p: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.c {
        clipboard::ai_request().await?;
    }

    if args.p {
        imageAnalyzer::ai_request().await?;
    }

    Ok(())
}

