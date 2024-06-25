    use pulldown_cmark::{Event, Parser, Tag, TagEnd};
    use termimad::{MadSkin, CompoundStyle, crossterm::style::StyledContent};
    use std::io::{self, Write};

    pub fn is_markdown(text: &str) -> bool {
        let parser = Parser::new(text);
        parser.into_iter().any(|event| match event {
            Event::Start(_) | Event::End(_) => true,
            _ => false,
        })
    }

    pub fn render_markdown(text: &str) {
        let skin = MadSkin::default();
        let mut buffer = Vec::new();
        format_markdown(text, &skin, &mut buffer);
        io::stdout().write_all(&buffer).unwrap();
    }

    pub fn format_markdown(text: &str, skin: &MadSkin, writer: &mut dyn Write)
  {
        let parser = Parser::new(text);
        let mut in_code_block = false;
        let mut code_block_content = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                    code_block_content.clear();
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let formatted =
  skin.inline_code.apply_to(code_block_content.as_str());
                    writeln!(writer, "{}", formatted).unwrap();
                    code_block_content.clear();
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else {
                        let formatted = skin.paragraph.compound_style.apply_to(&text);
                        write!(writer, "{}", formatted).unwrap();
                    }
                }
                Event::Code(text) => {
                    let formatted = skin.inline_code.apply_to(&text);
                    write!(writer, "{}", formatted).unwrap();
                }
                Event::SoftBreak => {
                    if in_code_block {
                        code_block_content.push('\n');
                    } else {
                        write!(writer, " ").unwrap();
                    }
                }
                Event::HardBreak => {
                    if in_code_block {
                        code_block_content.push('\n');
                    } else {
                        writeln!(writer).unwrap();
                    }
                }
                _ => {}
            }
        }
    }
