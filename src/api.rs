    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::env;
    use tokio_retry::{Retry, strategy::{ExponentialBackoff, jitter}};

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

    #[derive(Debug)]
    pub enum ApiError {
        RequestFailed(reqwest::Error),
        ResponseParseFailed(serde_json::Error),
        ApiErrorResponse(String),
        RetryExhausted,
    }

    impl std::fmt::Display for ApiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ApiError::RequestFailed(e) => write!(f, "Request failed: {}", e),
                ApiError::ResponseParseFailed(e) => write!(f, "Failed to parse response: {}", e),
                ApiError::ApiErrorResponse(e) => write!(f, "API error: {}", e),
                ApiError::RetryExhausted => write!(f, "Retry attempts exhausted"),
            }
        }
    }

    impl std::error::Error for ApiError {}

    async fn make_api_request(
        client: &Client,
        api_key: &str,
        model: &str,
        contents: &[String],
        no_markdown: bool,
    ) -> Result<String, ApiError> {
        let mut messages: Vec<serde_json::Value> = Vec::new();

        messages.push(serde_json::json!({
            "role": "system",
            "content": "You are a helpful coding tool. You should keep your
  answers brief, concise and mainly output code."
        }));

        // Add system message if Markdown is enabled
        if !no_markdown {
            messages.push(serde_json::json!({
                "role": "system",
                "content": "Please format your responses using Markdown syntax
  for better readability. Use appropriate Markdown elements for headers,
  lists, code blocks, and emphasis where applicable."
            }));
        }

        if no_markdown {
            messages.push(serde_json::json!({
                "role": "system",
                "content": "Answer without using markdown formatting!"
            }));
        }

        // Add user messages
        messages.extend(contents.iter().map(|content| {
            serde_json::json!({
                "role": "user",
                "content": content
            })
        }));

        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
        });

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(ApiError::RequestFailed)?;

        if !response.status().is_success() {
            let error_text = response.text().await.map_err(ApiError::RequestFailed)?;
            return Err(ApiError::ApiErrorResponse(error_text));
        }

        let response_text = response.text().await.map_err(ApiError::RequestFailed)?;
        let api_response: ApiResponse = serde_json::from_str(&response_text).map_err(ApiError::ResponseParseFailed)?;
        let content = api_response.choices.get(0)
            .map(|choice| choice.message.content.clone())
            .unwrap_or_default();
        Ok(content)
    }

    pub async fn send_api_request(
        client: &Client,
        api_key: &str,
        model: &str,
        contents: &[String],
        no_markdown: bool,
    ) -> Result<String, ApiError> {
        let retry_strategy = ExponentialBackoff::from_millis(100)
            .map(jitter)
            .take(3);

        Retry::spawn(retry_strategy, || async {
            make_api_request(client, api_key, model, contents, no_markdown).await
        })
        .await
        .map_err(|_| ApiError::RetryExhausted)
    }

    pub fn get_api_key() -> String {
        env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY not set")
    }
