use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestUserMessageArgs}};

use std::env; // initialises the paramters option 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(); 
    let prompt= env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: cargo run -- <text>");
        std::process::exit(1);
    });
    
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-5-mini")  // Using your model name
        .messages([
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .max_completion_tokens(4062_u32)
        .build()?;
    
    let response = client
        .chat()           // Just .chat()
        .create(request)  // Then .create() directly
        .await?;
    
    println!("{}", response.choices.first().unwrap().message.content.as_ref().unwrap());
    
    Ok(())
}
