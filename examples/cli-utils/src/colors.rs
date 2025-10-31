//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}
//// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", green("Green"));
/// ```     
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code for blue.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", blue("Blue"));
/// ```     
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code for bold text.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", bold("Bold"));               
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code to reset the color.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", reset("Reset"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}
/// Represents a color that can be applied to a string.
/// This enum defines the available colors that can be used for terminal output.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// let color = Color::Red;
/// println!("{}", red("This is red text"));
/// ```

pub enum Color {
    Red,
    Green,
    Blue,
    Bold,
}

/// Represents a colorized string with an associated color and opacity.
/// This struct can be used to create colorized output in the terminal.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// let mut color_string = ColorString {
///     color: Color::Red,
///     string: String::from("Hello, World!"),
///     colorized: String::new(),
///     opacity: 1.0,
/// };
/// color_string.paint();
/// println!("{}", color_string.colorized);
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String,
    pub opacity: f32,
}

impl ColorString {
    // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }
}
