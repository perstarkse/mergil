use std::env;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::process::Command;
use tempfile::NamedTempFile;

pub fn open_editor() -> Result<(String, NamedTempFile)> {
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    Command::new(&editor)
        .arg(&temp_path)
        .status()
        .expect("Failed to open editor");

    let mut file = File::open(&temp_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok((contents, temp_file))
}
#[allow(dead_code)]
pub fn write_test_data(data: &str) -> Result<NamedTempFile> {
    let temp_file = NamedTempFile::new()?;
    let mut file = temp_file.as_file();
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(temp_file)
}
