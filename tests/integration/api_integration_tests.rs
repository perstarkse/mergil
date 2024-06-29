use mergil::api;
use mergil::input::{self, EditorOpener, InputResult, StdinReader};
use reqwest::Client;
use std::cell::RefCell;

async fn setup_api_key() -> Option<String> {
    match std::env::var("OPENROUTER_API_KEY") {
        Ok(key) => Some(key),
        Err(_) => {
            println!("OPENROUTER_API_KEY not set. Skipping API tests.");
            None
        }
    }
}

struct MockStdin {
    content: RefCell<String>,
    is_atty: bool,
}

impl StdinReader for MockStdin {
    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let content = self.content.borrow();
        buf.push_str(&content);
        Ok(content.len())
    }

    fn is_atty(&self) -> bool {
        self.is_atty
    }
}

struct MockEditor {
    content: RefCell<String>,
}

impl EditorOpener for MockEditor {
    fn open_editor(&self, temp_path: &str) -> std::io::Result<()> {
        std::fs::write(temp_path, self.content.borrow().as_bytes())?;
        Ok(())
    }
}

#[tokio::test]
async fn test_markdown_api_request() {
    if let Some(api_key) = setup_api_key().await {
        let test_data = "Generate a simple Rust function that adds two numbers.";
        let contents_vec = vec![test_data.to_string()];

        let client = Client::new();
        let response = api::send_api_request(
            &client,
            &api_key,
            "deepseek/deepseek-coder",
            &contents_vec,
            true,
            None,
        )
        .await
        .unwrap();

        assert!(!response.is_empty());
        assert!(response.contains("```"));
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
            None,
        )
        .await
        .unwrap();

        assert!(!response.is_empty());
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

#[test]
fn test_get_input_piped() {
    let mut mock_stdin = MockStdin {
        content: RefCell::new("Test input\n".to_string()),
        is_atty: false,
    };
    let mock_editor = MockEditor {
        content: RefCell::new(String::new()),
    };

    let result = input::get_input(false, &mut mock_stdin, &mock_editor).unwrap();

    assert!(matches!(result, InputResult::Content(content) if content == "Test input\n"));
}

#[test]
fn test_get_input_force_editor() {
    let mut mock_stdin = MockStdin {
        content: RefCell::new(String::new()),
        is_atty: true,
    };
    let mock_editor = MockEditor {
        content: RefCell::new("Editor content".to_string()),
    };

    let result = input::get_input(true, &mut mock_stdin, &mock_editor).unwrap();

    assert!(matches!(result, InputResult::Content(content) if content == "Editor content"));
}

#[test]
fn test_get_input_force_editor_empty() {
    let mut mock_stdin = MockStdin {
        content: RefCell::new(String::new()),
        is_atty: true,
    };
    let mock_editor = MockEditor {
        content: RefCell::new("".to_string()),
    };

    let result = input::get_input(true, &mut mock_stdin, &mock_editor).unwrap();

    assert!(matches!(result, InputResult::Cancelled));
}
