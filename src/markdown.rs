    use pulldown_cmark::{Parser, Event};
    use termimad::MadSkin;

    pub fn is_markdown(text: &str) -> bool {
        let parser = Parser::new(text);
        parser.into_iter().any(|event| match event {
            Event::Start(_) | Event::End(_) => true,
            _ => false,
        })
    }

    pub fn render_markdown(text: &str) {
        let skin = MadSkin::default();
        skin.print_text(text);
    }

    pub fn format_markdown(text: &str) -> String {
        let parser = Parser::new(text);
        let mut formatted = String::new();

        for event in parser {
            match event {
                Event::Text(text) => formatted.push_str(&text),
                Event::Code(text) => formatted.push_str(&text),
                Event::SoftBreak | Event::HardBreak => formatted.push(' '),
                _ => {}
            }
        }

        formatted.trim().to_string()
    }
