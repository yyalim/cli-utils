use cli_utils::colors::{ ColorString, Color };

#[test]
fn test_red_coloring() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Hello".to_string(),
        colorized: "".to_string()
    };

    color_string.paint();

    assert_eq!(color_string.colorized.to_string(), "\x1b[31mHello\x1b[0m");
}