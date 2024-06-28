use termimad::*;

#[test]
fn test_create_madskin() {
    let skin = mergil::markdown::create_madskin();

    for header in &skin.headers {
        assert_eq!(header.compound_style.get_fg(), Some(rgb(100, 100, 100)));
    }

    assert_eq!(skin.bold.get_fg(), Some(rgb(255, 255, 0)));
    assert_eq!(skin.italic.get_fg(), Some(rgb(0, 0, 255)));
    assert_eq!(skin.italic.get_bg(), Some(rgb(0, 0, 0)));

    assert_eq!(
        skin.paragraph.compound_style.get_fg(),
        Some(rgb(200, 200, 200))
    );

    assert_eq!(skin.bullet.get_fg(), Some(rgb(255, 165, 0)));
    assert_eq!(skin.bullet.get_char(), '•');

    assert_eq!(skin.quote_mark.get_fg(), Some(rgb(150, 150, 150)));

    assert_eq!(skin.horizontal_rule.get_fg(), Some(rgb(100, 100, 100)));
    assert_eq!(skin.horizontal_rule.get_char(), '─');

    assert_eq!(
        skin.code_block.compound_style.get_bg(),
        Some(rgb(30, 30, 30))
    );
}
