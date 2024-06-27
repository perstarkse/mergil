use mergil::api::{self, ApiError};
use std::env;
use std::time::Duration;
use tokio::time::timeout;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_send_api_request_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{
                "message": {
                    "content": "Hello, world!"
                }
            }]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

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

    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Bad request"))
        .expect(1)
        .mount(&mock_server)
        .await;

    env::set_var("OPENROUTER_API_KEY", "test_key");
    let client = reqwest::Client::new();
    let result = timeout(
        Duration::from_secs(5),
        api::send_api_request(
            &client,
            "test_key",
            "test-model",
            &vec!["Hello".to_string()],
            false,
            Some(&mock_server.uri()),
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

    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal server error"))
        .expect(2)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{
                "message": {
                    "content": "Success after retry!"
                }
            }]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    env::set_var("OPENROUTER_API_KEY", "test_key");
    let client = reqwest::Client::new();
    let result = timeout(
        Duration::from_secs(5),
        api::send_api_request(
            &client,
            "test_key",
            "test-model",
            &vec!["Hello".to_string()],
            false,
            Some(&mock_server.uri()),
        ),
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), "Success after retry!");
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
