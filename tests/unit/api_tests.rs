use mergil::api::{self, ApiError};
use std::env;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[test]
fn test_get_api_key() {
    env::set_var("OPENROUTER_API_KEY", "test_key");
    assert_eq!(api::get_api_key(), "test_key");
}

#[tokio::test]
async fn test_send_api_request_success() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [{
                    "message": {
                        "content": "Hello, world!"
                    }
                }]
            })))
        .mount(&mock_server)
        .await;

    env::set_var("OPENROUTER_API_KEY", "test_key");
    let client = reqwest::Client::new();
    let result = api::send_api_request(&client, "test_key", "test-model", &vec!["Hello".to_string()], false).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, world!");
}

#[tokio::test]
async fn test_send_api_request_error() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/api/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(400)
            .set_body_string("Bad request"))
        .mount(&mock_server)
        .await;

    env::set_var("OPENROUTER_API_KEY", "test_key");
    let client = reqwest::Client::new();
    let result = api::send_api_request(&client, "test_key", "test-model", &vec!["Hello".to_string()], false).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ApiError::ApiErrorResponse(_)));
}

#[test]
fn test_api_error_display() {
    // let request_error = ApiError::RequestFailed(ApiError::ApiErrorResponse("Error".to_string()));
    // assert!(format!("{}", request_error).contains("Request failed"));

    let parse_error = ApiError::ResponseParseFailed(serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err());
    assert!(format!("{}", parse_error).contains("Failed to parse response"));

    let api_error = ApiError::ApiErrorResponse("Bad request".to_string());
    assert_eq!(format!("{}", api_error), "API error: Bad request");

    let retry_error = ApiError::RetryExhausted;
    assert_eq!(format!("{}", retry_error), "Retry attempts exhausted");
}
