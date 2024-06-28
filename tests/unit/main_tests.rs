use assert_cmd::Command;
use std::str;

#[test]
fn test_cli_with_context() {
    let mut cmd = Command::cargo_bin("mergil").unwrap();
    let output = cmd
        .arg("Hello, world!")
        .arg("--debug")
        .env("RUST_TEST", "1")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Hello, world!"));
}

#[test]
fn test_cli_with_model_flag() {
    let mut cmd = Command::cargo_bin("mergil").unwrap();
    let output = cmd
        .arg("-m")
        .arg("gpt-3.5-turbo")
        .arg("Test message")
        .env("RUST_TEST", "1")
        .arg("--debug")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Model: gpt-3.5-turbo"));
    assert!(stdout.contains("Test message"));
}

#[test]
fn test_cli_with_debug_flag() {
    let mut cmd = Command::cargo_bin("mergil").unwrap();
    let output = cmd
        .arg("--debug")
        .arg("Debug test")
        .env("RUST_TEST", "1")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Input content:"));
    assert!(stdout.contains("Debug test"));
}

#[test]
fn test_cli_with_markdown_flag() {
    let mut cmd = Command::cargo_bin("mergil").unwrap();
    let output = cmd
        .arg("--markdown")
        .arg("--debug")
        .arg("# Markdown test")
        .env("RUST_TEST", "1")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Markdown: true"));
    assert!(stdout.contains("# Markdown test"));
}

#[test]
fn test_cli_with_piped_input() {
    let mut cmd = Command::cargo_bin("mergil").unwrap();
    let output = cmd
        .arg("--debug")
        .write_stdin("Piped input test")
        .env("RUST_TEST", "1")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Input content:"));
    assert!(stdout.contains("Piped input test"));
}

#[test]
fn test_cli_with_no_input() {
    let mut cmd = Command::cargo_bin("mergil").unwrap();
    let output = cmd
        .arg("--debug")
        .env("RUST_TEST", "1")
        .env("NO_EDITOR", "1") // Set this environment variable to skip
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("No input provided. Exiting."));
}
