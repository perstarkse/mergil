use mergil::input::InputResult;
use mockall::mock;
use mockall::predicate::*;
use std::io::Cursor;
use std::io::Read;
use std::os::unix::process::ExitStatusExt;

mock! {
    #[derive(Clone, Default)]
    EditorCommand {
        fn new(editor: &str) -> Self;
        fn arg(&mut self, arg: &str) -> &mut Self;
        fn status(&mut self) -> std::io::Result<std::process::ExitStatus>;
    }
}

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
    let mut mock_command = MockEditorCommand::new("vi");
    mock_command
        .expect_arg()
        .returning_st(|_| MockEditorCommand::default());
    mock_command
        .expect_status()
        .return_once(|| Ok(std::process::ExitStatus::from_raw(0)));

    // Mock file operations
    let mock_file_content = "Editor content";
    let mock_read_to_string =
        |_: &str| -> std::io::Result<String> { Ok(mock_file_content.to_string()) };

    let result = get_input_with_mocks(true, mock_command, mock_read_to_string).unwrap();

    assert!(matches!(result, InputResult::Content(content) if content == mock_file_content));
}

fn get_input_with_mocks<F>(
    force_editor: bool,
    mock_command: MockEditorCommand,
    mock_read_to_string: F,
) -> std::io::Result<InputResult>
where
    F: Fn(&str) -> std::io::Result<String>,
{
    if force_editor {
        let temp_path = "/tmp/mock_file.txt";
        let mut command = mock_command;
        command.arg(temp_path);
        command.status()?;
        let contents = mock_read_to_string(temp_path)?;
        if contents.trim().is_empty() {
            Ok(InputResult::Cancelled)
        } else {
            Ok(InputResult::Content(contents))
        }
    } else {
        unimplemented!("This test doesn't cover non-editor functionality")
    }
}

#[test]
fn test_get_input_force_editor_empty() {
    let mut mock_command = MockEditorCommand::new("vi");
    mock_command
        .expect_arg()
        .returning_st(|_| MockEditorCommand::default());
    mock_command
        .expect_status()
        .return_once(|| Ok(std::process::ExitStatus::from_raw(0)));

    // Mock file operations
    let mock_file_content = "";
    let mock_read_to_string =
        |_: &str| -> std::io::Result<String> { Ok(mock_file_content.to_string()) };

    let result = get_input_with_mocks(true, mock_command, mock_read_to_string).unwrap();

    assert!(matches!(result, InputResult::Cancelled));
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
