use mergil::common::{handle_input, process_contents, Cli};
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
async fn test_handle_input_with_piped_input() {
    let cli = Cli {
        context: vec!["Piped input test".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let mut input = io::Cursor::new(Vec::new());
    input.write_all(b"Piped input test").unwrap();
    input.seek(io::SeekFrom::Start(0)).unwrap();

    let contents = handle_input(&cli).await.unwrap();
    assert_eq!(contents, vec!["Piped input test".to_string()]);
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
#[tokio::test]
async fn test_process_contents_debug_flag() {
    env::set_var("RUST_TEST", "1");
    let cli = Cli {
        context: vec!["Debug test".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = vec!["Debug test".to_string()];
    process_contents(&cli, &contents).await.unwrap();
    env::remove_var("RUST_TEST");
}

#[tokio::test]
async fn test_process_contents_markdown_flag() {
    env::set_var("RUST_TEST", "1");
    let cli = Cli {
        context: vec!["Markdown test".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: true,
    };

    let contents = vec!["Markdown test".to_string()];
    process_contents(&cli, &contents).await.unwrap();
    env::remove_var("RUST_TEST");
}

#[tokio::test]
async fn test_process_contents_no_input() {
    env::set_var("RUST_TEST", "1");
    let cli = Cli {
        context: vec![],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = Vec::<String>::new();
    process_contents(&cli, &contents).await.unwrap();
    env::remove_var("RUST_TEST");
}

#[tokio::test]
async fn test_process_contents_multiple_inputs() {
    env::set_var("RUST_TEST", "1");
    let cli = Cli {
        context: vec!["Input 1".to_string(), "Input 2".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = vec!["Input 1".to_string(), "Input 2".to_string()];
    process_contents(&cli, &contents).await.unwrap();
    env::remove_var("RUST_TEST");
}

#[tokio::test]
async fn test_process_contents_skip_api_call() {
    env::set_var("RUST_TEST", "1");
    let cli = Cli {
        context: vec!["Test API skip".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
    };

    let contents = vec!["Test API skip".to_string()];
    process_contents(&cli, &contents).await.unwrap();
    env::remove_var("RUST_TEST");
}
