// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod imageAnalyzer;
mod api;

use tauri::{Manager, Emitter};
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long = "clipboard")]
    c: bool,
    #[arg(short = 'p', long = "picture")]
    p: bool,
    #[arg(short = 'd', long = "directory", value_name = "DIR", default_value = "")]
    d: String,
}

#[tauri::command]
async fn analyze_image(path: String) -> Result<String, String> {
    imageAnalyzer::ai_request(&path)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    // Parse CLI args
    let args = Args::parse();
    
    println!("Args parsed: {:?}", args);
    println!("Picture flag: {}", args.p);
    println!("Directory: '{}'", args.d);
    
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            
            // If image path provided via CLI, trigger analysis
            if args.p && !args.d.is_empty() {
                println!("Analyzing image: {}", args.d);
                let path = args.d.clone();
                let window_clone = window.clone();
                
                tauri::async_runtime::spawn(async move {
                    println!("Starting analysis for: {}", path);
                    match imageAnalyzer::ai_request(&path).await {
                        Ok(result) => {
                            println!("Analysis complete! Result length: {} chars", result.len());
                            println!("Result preview: {}...", &result[..result.len().min(100)]);
                            
                            // Try to emit the event
                            match window_clone.emit("analysis-complete", &result) {
                                Ok(_) => println!("✓ Event emitted successfully"),
                                Err(e) => eprintln!("✗ Failed to emit event: {}", e),
                            }
                        }
                        Err(e) => {
                            eprintln!("Error analyzing image: {}", e);
                            match window_clone.emit("analysis-error", e.to_string()) {
                                Ok(_) => println!("✓ Error event emitted successfully"),
                                Err(e) => eprintln!("✗ Failed to emit error event: {}", e),
                            }
                        }
                    }
                });
            } else {
                println!("No image provided or flags not set correctly");
                println!("Use: cargo tauri dev -- -- -p -d <image_path>");
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![analyze_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
