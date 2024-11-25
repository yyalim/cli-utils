//! ANSI color codes for use in the terminal. With helper functions to wrap strings in color codes.
//! # Examples
//!  ```
//! use cli_utils::colors::{red, green, blue};
//! let red = red("Hello, world!");
//! let green = green("Hello, world!");
//! let blue = blue("Hello, world!");
//! ```

/// Returns a string with the ANSI color code for red
/// # Examples
/// ```
/// use cli_utils::colors::red;
/// let red = cli_utils::colors::red("Hello, world!");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string with the ANSI color code for green
/// # Examples
/// ```
///  use cli_utils::colors::green;
///  let green = cli_utils::colors::green("Hello, world!");
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI color code for blue
/// # Examples
/// ```
/// use cli_utils::colors::blue;
/// let blue = cli_utils::colors::blue("Hello, world!");
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI code for bold text
/// # Examples
/// ```
/// use cli_utils::colors::bold;
/// let bold = cli_utils::colors::bold("Hello, world!");
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Resets text formatting to default
/// # Examples
/// ```
/// use cli_utils::colors::reset;
/// let reset = cli_utils::colors::reset("Hello, world!");
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Enum representing the possible colors for text
/// # Examples
/// ```
/// use cli_utils::colors::Color;
/// let color = Color::Red;
/// ```
pub enum Color {
    Red,
    Green,
    Blue,
    Bold
}

/// Struct representing a string with a color
/// # Examples
/// ```
/// use cli_utils::colors::{ColorString, Color};
/// let color_string = ColorString {
///     color: Color::Red,
///     string: "Hello, world!".to_string(),
///     colorized: "".to_string()
/// };
/// ```
/// Paint the string with the color
/// ```
/// use cli_utils::colors::{ColorString, Color};
/// let mut color_string = ColorString {
///   color: Color::Red,
///   string: "Hello, world!".to_string(),
///   colorized: "".to_string()
/// };
/// color_string.paint();
/// ```
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

impl ColorString {
  pub fn paint(&mut self) {
     match self.color {
         Color::Red => self.colorized = red(&self.string),
         Color::Green => self.colorized = green(&self.string),
         Color::Blue => self.colorized = blue(&self.string),
         Color::Bold => self.colorized = bold(&self.string),
     }
  }
  pub fn reset(&mut self) {
      self.colorized = reset(&self.string);
  }
}
