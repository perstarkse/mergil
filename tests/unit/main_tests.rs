use mergil::common::{handle_input, Cli};
use std::{
    env,
    io::{self, Write},
};
use std_prelude::Seek;

#[tokio::test]
async fn test_handle_input_with_context() {
    let cli = Cli {
        context: vec!["Hello, world!".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, vec!["Hello, world!".to_string()]);
}

#[tokio::test]
async fn test_handle_input_no_input_provided() {
    let cli = Cli {
        context: vec![],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, Vec::<String>::new());
}

#[tokio::test]
async fn test_handle_input_empty_context() {
    env::set_var("NO_EDITOR", "1");
    let cli = Cli {
        context: vec![],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, Vec::<String>::new());
}

#[tokio::test]
async fn test_handle_input_multiple_context_entries() {
    let cli = Cli {
        context: vec!["Hello".to_string(), "world!".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, vec!["Hello world!".to_string()]);
}

#[tokio::test]
async fn test_handle_input_debug_flag() {
    let cli = Cli {
        context: vec!["Debug test".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, vec!["Debug test".to_string()]);
}

#[tokio::test]
async fn test_handle_input_markdown_flag() {
    let cli = Cli {
        context: vec!["Markdown test".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: true,
    };

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, vec!["Markdown test".to_string()]);
}

#[tokio::test]
async fn test_handle_input_empty_piped_input() {
    let cli = Cli {
        context: vec!["Command line input".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let mut input = io::Cursor::new(Vec::new());
    input.write_all(b"").unwrap();
    input.seek(io::SeekFrom::Start(0)).unwrap();

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, vec!["Command line input".to_string()]);
}
