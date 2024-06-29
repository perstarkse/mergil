use mergil::input::{get_input, EditorOpener, InputResult, StdinReader};
use std::{cell::RefCell, io};

struct MockStdin {
    content: RefCell<String>,
    is_atty: bool,
}

impl StdinReader for MockStdin {
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
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
    fn open_editor(&self, temp_path: &str) -> io::Result<()> {
        std::fs::write(temp_path, self.content.borrow().as_bytes())?;
        Ok(())
    }
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

    let result = get_input(false, &mut mock_stdin, &mock_editor).unwrap();

    assert!(matches!(result, InputResult::Content(content) if content == "Test input\n"));
}

#[test]
fn test_get_input_piped_empty() {
    let mut mock_stdin = MockStdin {
        content: RefCell::new("\n".to_string()),
        is_atty: false,
    };
    let mock_editor = MockEditor {
        content: RefCell::new(String::new()),
    };

    let result = get_input(false, &mut mock_stdin, &mock_editor).unwrap();

    assert!(matches!(result, InputResult::Cancelled));
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

    let result = get_input(true, &mut mock_stdin, &mock_editor).unwrap();

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

    let result = get_input(true, &mut mock_stdin, &mock_editor).unwrap();

    assert!(matches!(result, InputResult::Cancelled));
}
