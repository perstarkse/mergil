use mergil::common::{handle_input, Cli};
use std::io::{self, Write};
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
