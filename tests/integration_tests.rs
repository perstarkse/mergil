use mergil::api;
use mergil::editor;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

#[tokio::test]
async fn test_api_request() {
    let test_data = "This is a test input.";
    let temp_file = editor::write_test_data(test_data).unwrap();
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    let mut file = File::open(&temp_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    assert_eq!(contents, test_data);

    let api_key = api::get_api_key();
    let client = Client::new();
    let response = api::send_api_request(
        &client,
        &api_key,
        "deepseek/deepseek-coder",
        &contents,
    )
    .await
    .unwrap();

    assert!(!response.choices.is_empty());
}
