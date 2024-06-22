use pulldown_cmark::{Parser, Event};
use termimad::{MadSkin, StyledChar};

pub fn is_markdown(text: &str) -> bool {
    let parser = Parser::new(text);
    parser.into_iter().any(|event| match event {
        Event::Start(_) | Event::End(_) => true,
        _ => false,
    })
}

pub fn render_markdown(text: &str) {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(termimad::rgb(255, 187, 0));
    skin.bold.set_fg(termimad::rgb(255, 187, 0));
    skin.italic.set_fg(termimad::rgb(215, 215, 215));
    skin.bullet = StyledChar::from_fg_char(termimad::rgb(215, 95, 0), '•');

    skin.print_text(text);
}
