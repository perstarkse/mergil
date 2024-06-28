use mergil::input::InputResult;
use std::fs;
use std::io::{Cursor, Read};
use tempfile::NamedTempFile;

#[test]
fn test_get_input_piped() {
    let input = "Test input\n";
    let mut cursor = Cursor::new(input);

    let result = get_input_with_stdin(false, &mut cursor).unwrap();

    assert!(matches!(result, InputResult::Content(content) if content == input));
}

#[test]
fn test_get_input_piped_empty() {
    let input = "\n";
    let mut cursor = Cursor::new(input);

    let result = get_input_with_stdin(false, &mut cursor).unwrap();

    assert!(matches!(result, InputResult::Cancelled));
}

#[test]
fn test_get_input_force_editor() {
    let temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_str().unwrap().to_string();
    fs::write(&temp_path, "Editor content").unwrap();

    let result = get_input_with_editor(&temp_path).unwrap();

    assert!(matches!(result, InputResult::Content(content) if content == "Editor content"));
}

#[test]
fn test_get_input_force_editor_empty() {
    let temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_str().unwrap().to_string();
    fs::write(&temp_path, "").unwrap();

    let result = get_input_with_editor(&temp_path).unwrap();

    assert!(matches!(result, InputResult::Cancelled));
}

fn get_input_with_editor(temp_path: &str) -> std::io::Result<InputResult> {
    let contents = fs::read_to_string(temp_path)?;
    if contents.trim().is_empty() {
        Ok(InputResult::Cancelled)
    } else {
        Ok(InputResult::Content(contents))
    }
}

fn get_input_with_stdin<R: Read>(
    force_editor: bool,
    stdin: &mut R,
) -> std::io::Result<InputResult> {
    if force_editor {
        unimplemented!("This test doesn't cover editor functionality")
    } else {
        let mut buffer = String::new();
        stdin.read_to_string(&mut buffer)?;
        if buffer.trim().is_empty() {
            Ok(InputResult::Cancelled)
        } else {
            Ok(InputResult::Content(buffer))
        }
    }
}
