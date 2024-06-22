use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub id: String,
    pub model: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub system_fingerprint: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u64,
    pub message: Message,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: ErrorDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorDetails {
    message: String,
    code: u32,
}

pub async fn send_api_request(
    client: &Client,
    api_key: &str,
    model: &str,
    contents: &[String],
) -> Result<ApiResponse, String> {
    let messages: Vec<serde_json::Value> = contents
        .iter()
        .map(|content| {
            serde_json::json!({
                "role": "user",
                "content": content
            })
        })
        .collect();

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": model,
            "messages": messages
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let response_text = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Try to parse as an error response first
    if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
        return Err(format!("API Error: {}", error_response.error.message));
    }

    // If it's not an error response, try to parse as the expected ApiResponse
    serde_json::from_str::<ApiResponse>(&response_text)
        .map_err(|e| format!("Failed to parse API response: {}", e))
}

pub fn get_api_key() -> String {
    env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY not set")
}
