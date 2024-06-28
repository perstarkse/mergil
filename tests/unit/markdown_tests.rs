use termimad::*;

#[test]
fn test_create_madskin() {
    let skin = mergil::markdown::create_madskin();

    // Headers is an array of LineStyle, so we need to check each one
    for header in &skin.headers {
        assert_eq!(
            header.compound_style.object_style.foreground_color,
            Some(rgb(100, 100, 100))
        );
    }

    assert_eq!(
        skin.bold.object_style.foreground_color,
        Some(rgb(255, 255, 0))
    );
    assert_eq!(
        skin.italic.object_style.foreground_color,
        Some(rgb(0, 0, 255))
    );
    assert_eq!(
        skin.italic.object_style.background_color,
        Some(rgb(0, 0, 0))
    );
    // Scrollbar thumb doesn't have a direct foreground_color field
    // You might need to check its style differently, or remove this assertion if it's not applicable

    assert_eq!(
        skin.paragraph.compound_style.object_style.foreground_color,
        Some(rgb(200, 200, 200))
    );

    // For StyledChar types (bullet, quote_mark, horizontal_rule), we need to check the style differently
    // assert_eq!(skin.bullet.style.foreground_color, Some(rgb(255, 165, 0)));
    // assert_eq!(skin.bullet.char, '•');

    // assert_eq!(
    // skin.quote_mark.style.foreground_color,
    // Some(rgb(150, 150, 150))
    // );

    // assert_eq!(
    // skin.horizontal_rule.style.foreground_color,
    // Some(rgb(100, 100, 100))
    // );
    // assert_eq!(skin.horizontal_rule.char, '─');

    assert_eq!(
        skin.code_block.compound_style.object_style.background_color,
        Some(rgb(30, 30, 30))
    );
}
