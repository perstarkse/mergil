use mergil::common::Cli;
use std::env;

#[tokio::test]
async fn test_run() {
    env::set_var("RUST_TEST", "1");

    let cli = Cli {
        context: vec!["Test input".to_string()],
        model: "deepseek/deepseek-coder".to_string(),
        debug: true,
        markdown: false,
        preprocess: false,
    };

    let result = mergil::run(cli).await;
    assert!(result.is_ok());

    env::remove_var("RUST_TEST");
}
