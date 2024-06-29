use atty::Stream;
use std::io::{self, Read};
use tempfile::NamedTempFile;

pub enum InputResult {
    Content(String),
    Cancelled,
}

pub trait StdinReader {
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>;
    fn is_atty(&self) -> bool;
}

pub trait EditorOpener {
    fn open_editor(&self, temp_path: &str) -> io::Result<()>;
}

pub struct RealStdin;

impl StdinReader for RealStdin {
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        io::stdin().read_to_string(buf)
    }

    fn is_atty(&self) -> bool {
        atty::is(Stream::Stdin)
    }
}

pub struct RealEditor;

impl EditorOpener for RealEditor {
    fn open_editor(&self, temp_path: &str) -> io::Result<()> {
        use std::env;
        use std::process::Command;

        let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        Command::new(&editor).arg(temp_path).status().map(|_| ())
    }
}

pub fn get_input(
    force_editor: bool,
    stdin_reader: &mut dyn StdinReader,
    editor_opener: &dyn EditorOpener,
) -> io::Result<InputResult> {
    if force_editor || stdin_reader.is_atty() {
        open_editor(editor_opener)
    } else {
        let mut buffer = String::new();
        stdin_reader.read_to_string(&mut buffer)?;
        if buffer.trim().is_empty() {
            Ok(InputResult::Cancelled)
        } else {
            Ok(InputResult::Content(buffer))
        }
    }
}

fn open_editor(editor_opener: &dyn EditorOpener) -> io::Result<InputResult> {
    use std::fs::{self, File};

    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    File::create(&temp_path)?;

    editor_opener.open_editor(&temp_path)?;

    let contents = fs::read_to_string(&temp_path)?;

    if contents.trim().is_empty() {
        Ok(InputResult::Cancelled)
    } else {
        Ok(InputResult::Content(contents))
    }
}
