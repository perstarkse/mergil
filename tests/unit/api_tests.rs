use mergil::api::{self, ApiError};
use std::env;
use std::time::Duration;
use tokio::time::timeout;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub async fn mock_successful_api_response(mock_server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "test-id",
            "model": "test-model",
            "object": "object-test",
            "created": 10010,
            "system_fingerprint": "none",
            "usage": {
                "prompt_tokens": 50,
                "completion_tokens": 60,
                "total_tokens": 110
            },
            "choices": [{
                "finish_reason": "stop",
                "index": 1,
                "message": {
                    "role": "assistant",
                    "content": "Hello, world!"
                }
            }]
        })))
        .expect(1)
        .mount(mock_server)
        .await;
}

pub async fn mock_error_api_response(mock_server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Bad request"))
        .expect(3)
        .mount(mock_server)
        .await;
}

#[tokio::test]
async fn test_send_api_request_success() {
    let mock_server = MockServer::start().await;

    mock_successful_api_response(&mock_server).await;

    env::set_var("OPENROUTER_API_KEY", "test_key");
    let url = format!("{}/api/v1/chat/completions", &mock_server.uri());
    let client = reqwest::Client::new();
    let result = timeout(
        Duration::from_secs(5),
        api::send_api_request(
            &client,
            "test_key",
            "test-model",
            &vec!["Hello".to_string()],
            false,
            Some(&url),
        ),
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), "Hello, world!");
}

#[tokio::test]
async fn test_send_api_request_error() {
    let mock_server = MockServer::start().await;
    mock_error_api_response(&mock_server).await;

    env::set_var("OPENROUTER_API_KEY", "test_key");
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/chat/completions", &mock_server.uri());
    let result = timeout(
        Duration::from_secs(5),
        api::send_api_request(
            &client,
            "test_key",
            "test-model",
            &vec!["Hello".to_string()],
            false,
            Some(&url),
        ),
    )
    .await;

    assert!(result.is_ok());
    let error = result.unwrap().unwrap_err();
    assert!(matches!(error, ApiError::ApiErrorResponse(_)));
}

#[tokio::test]
async fn test_send_api_request_retry() {
    let mock_server = MockServer::start().await;

    // mock_error_api_response(&mock_server).await;
    mock_successful_api_response(&mock_server).await;
    env::set_var("OPENROUTER_API_KEY", "test_key");
    let url = format!("{}/api/v1/chat/completions", &mock_server.uri());
    let client = reqwest::Client::new();
    let result = timeout(
        Duration::from_secs(5),
        api::send_api_request(
            &client,
            "test_key",
            "test-model",
            &vec!["Hello".to_string()],
            false,
            Some(&url),
        ),
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), "Hello, world!");
}

#[test]
fn test_api_error_display() {
    // let request_error = ApiError::RequestFailed(reqwest::Error::is_timeout("timeout"));
    // assert!(format!("{}", request_error).contains("Request failed"));

    let parse_error = ApiError::ResponseParseFailed(
        serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err(),
    );
    assert!(format!("{}", parse_error).contains("Failed to parse response"));

    let api_error = ApiError::ApiErrorResponse("Bad request".to_string());
    assert_eq!(format!("{}", api_error), "API error: Bad request");

    let retry_error = ApiError::RetryExhausted;
    assert_eq!(format!("{}", retry_error), "Retry attempts exhausted");
}
