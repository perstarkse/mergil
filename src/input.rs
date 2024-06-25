use std::io::{self, Read, Result, Write};
use atty::Stream;
use tempfile::NamedTempFile;

pub enum InputResult {
    Content(String),
    Cancelled,
}

    pub fn get_input(force_editor: bool) -> io::Result<InputResult> {
        if force_editor || atty::is(Stream::Stdin) {
            // If force_editor is true or no input is piped, open the editor
            open_editor()
        } else {
            // If input is piped, read from STDIN
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            if buffer.trim().is_empty() {
                Ok(InputResult::Cancelled)
            } else {
                Ok(InputResult::Content(buffer))
            }
        }
    }

fn open_editor() -> io::Result<InputResult> {
    use std::env;
    use std::fs::{self, File};
    use std::process::Command;

    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    // Create an empty file
    File::create(&temp_path)?;

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    Command::new(&editor)
        .arg(&temp_path)
        .status()
        .expect("Failed to open editor");

    // Read the file contents after the editor is closed
    let contents = fs::read_to_string(&temp_path)?;

    // Check if the content is empty
    if contents.trim().is_empty() {
        Ok(InputResult::Cancelled)
    } else {
        Ok(InputResult::Content(contents))
    }
}

#[allow(dead_code)]
pub fn write_test_data(data: &str) -> Result<NamedTempFile> {
    let temp_file = NamedTempFile::new()?;
    let mut file = temp_file.as_file();
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(temp_file)
}
