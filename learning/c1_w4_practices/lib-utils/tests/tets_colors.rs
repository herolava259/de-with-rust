use lib_utils::colors::{ColorString, Color};
use rstest::rstest;


#[rstest]
#[case(Color::Red, "Red", "\x1b[31mRed\x1b[0m")]
#[case(Color::Green, "Green", "\x1b[32mGreen\x1b[0m")]
#[case(Color::Blue, "Blue", "\x1b[34mBlue\x1b[0m")]
fn test_color_string(#[case] color: Color, #[case] string: &str, #[case] expected: &str) {
    
    let mut color_string = ColorString {
        color,
        string: string.to_string(),
        colorized: String::new(),
    };
    color_string.paint();

    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m")
}