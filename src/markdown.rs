use termimad::*;

pub fn create_madskin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(100, 100, 100));
    skin.bold.set_fg(rgb(255, 255, 0));
    skin.italic.set_fgbg(rgb(0, 0, 255), rgb(0, 0, 0));
    skin.scrollbar.thumb.set_fg(rgb(0, 255, 0));
    skin.code_block.align = Alignment::Left;

    // Additional sensible defaults
    skin.paragraph.set_fg(rgb(200, 200, 200));
    skin.bullet = StyledChar::from_fg_char(rgb(255, 165, 0), '•');
    skin.quote_mark.set_fg(rgb(150, 150, 150));
    skin.horizontal_rule = StyledChar::from_fg_char(rgb(100, 100, 100), '─');
    skin.code_block.set_bg(rgb(30, 30, 30));

    skin
}
