use std::env;
use std::fs::File;
use std::io::{Read, Result};
use std::process::Command;
use tempfile::NamedTempFile;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a temporary file
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    // Get the default editor from the environment or fallback to "vi"
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    // Open the editor
    Command::new(&editor)
        .arg(&temp_path)
        .status()
        .expect("Failed to open editor");

    // Read the contents of the file
    let mut file = File::open(&temp_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Print the contents
    println!("You entered:\n{}", contents);

    // Get the API key from environment variables
    let api_key = env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY not set");

    // Prepare the API request
    let client = Client::new();
    let response = client.post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "deepseek/deepseek-coder",
            "messages": [
                {"role": "user", "content": contents}
            ]
        }))
        .send()
        .await
        .expect("Failed to send request");

    // Parse the response
    let response_text = response.text().await.expect("Failed to read response text");
    let json_response: Value = serde_json::from_str(&response_text).expect("Failed to parse JSON");

    // Extract and print the "content" field
    if let Some(content) = json_response["choices"][0]["message"]["content"].as_str() {
        // Ensure the content does not start with an empty space
        let trimmed_content = content.trim_start();
        println!("API Response:\n{}", trimmed_content);
    } else {
        println!("Failed to extract content from the API response");
    }

    Ok(())
}
