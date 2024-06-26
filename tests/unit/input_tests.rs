use mergil::input::{self, InputResult};
use std::io::Cursor;

#[test]
fn test_get_input_from_stdin() {
    let input = "Test input\n";
    let _cursor = Cursor::new(input);
    let result = input::get_input(false).unwrap();

    assert!(matches!(result, InputResult::Content(_)));
    if let InputResult::Content(content) = result {
        assert_eq!(content.trim(), "Test input");
    }
}

#[test]
fn test_get_input_cancelled() {
    let input = "\n";
    let _cursor = Cursor::new(input);
    let result = input::get_input(false).unwrap();

    assert!(matches!(result, InputResult::Cancelled));
}

#[test]
fn test_write_test_data() {
    let test_data = "Hello, world!";
    let result = input::write_test_data(test_data);
    assert!(result.is_ok());

    let temp_file = result.unwrap();
    let content = std::fs::read_to_string(temp_file.path()).unwrap();
    assert_eq!(content, test_data);
}
