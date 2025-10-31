

#[cfg(test)]
mod tests {
    use cli_utils::colors::{Color, ColorString};
    use cli_utils::read_stdin;
    #[test]
    fn test_red_coloring() {
        let mut color_string = ColorString {
            color: Color::Red,
            string: "Red".to_string(),
            colorized: "".to_string(),
            opacity: 1.0
        };
        color_string.paint();
        assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
    }

    #[test]
    fn test_green_coloring() {
        let mut color_string = ColorString {
            color: Color::Green,
            string: "Green".to_string(),
            colorized: "".to_string(),
            opacity: 1.0
        };
        color_string.paint();
        assert_eq!(color_string.colorized, "\x1b[32mGreen\x1b[0m");
    }

    #[test]
    fn test_blue_coloring() {
        let mut color_string = ColorString {
            color: Color::Blue,
            string: "Blue".to_string(),
            colorized: "".to_string(),
            opacity: 1.0
        };
        color_string.paint();
        assert_eq!(color_string.colorized, "\x1b[34mBlue\x1b[0m");
    }

    #[test]
    fn test_bold_coloring() {
        let mut color_string = ColorString {
            color: Color::Bold,
            string: "Bold".to_string(),
            colorized: "".to_string(),
            opacity: 1.0
        };
        color_string.paint();
        assert_eq!(color_string.colorized, "\x1b[1mBold\x1b[0m");
    }
}
