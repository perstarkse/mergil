use mergil::api;
use mergil::input;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

async fn setup_api_key() -> Option<String> {
    match std::env::var("OPENROUTER_API_KEY") {
        Ok(key) => Some(key),
        Err(_) => {
            println!("OPENROUTER_API_KEY not set. Skipping API tests.");
            None
        }
    }
}

#[tokio::test]
async fn test_api_request() {
    if let Some(api_key) = setup_api_key().await {
        let test_data = "This is a test input.";
        let temp_file = input::write_test_data(test_data).unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let mut file = File::open(&temp_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, test_data);

        let contents_vec = vec![contents];

        let client = Client::new();
        let response = api::send_api_request(
            &client,
            &api_key,
            "deepseek/deepseek-coder",
            &contents_vec,
            false,
        )
        .await
        .unwrap();

        assert!(!response.is_empty());
    }
}

#[tokio::test]
async fn test_markdown_api_request() {
    if let Some(api_key) = setup_api_key().await {
        let test_data = "Generate a simple Rust function that adds two
  numbers.";
        let contents_vec = vec![test_data.to_string()];

        let client = Client::new();
        let response = api::send_api_request(
            &client,
            &api_key,
            "deepseek/deepseek-coder",
            &contents_vec,
            true,
        )
        .await
        .unwrap();

        assert!(!response.is_empty());
        assert!(response.contains("```rust"));
    }
}

#[tokio::test]
async fn test_multiple_inputs() {
    if let Some(api_key) = setup_api_key().await {
        let input1 = "What is a closure in Rust?";
        let input2 = "Provide an example of using a closure.";
        let contents_vec = vec![input1.to_string(), input2.to_string()];

        let client = Client::new();
        let response = api::send_api_request(
            &client,
            &api_key,
            "deepseek/deepseek-coder",
            &contents_vec,
            false,
        )
        .await
        .unwrap();

        assert!(!response.is_empty());
        assert!(response.contains("closure"));
    }
}

#[test]
fn test_missing_api_key() {
    std::env::remove_var("OPENROUTER_API_KEY");
    let result = std::panic::catch_unwind(|| {
        api::get_api_key();
    });
    assert!(result.is_err());
}
