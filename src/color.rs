use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum Color {
    Red = 31,
    Yellow = 33,
    Blue = 34,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(*self as usize).to_string())
    }
}

pub trait Colorize: Display {
    fn to_color(&self, col: Color) -> String {
        format!("\x1b[{col}m{}\x1b[0m", self)
    }
}

impl<T> Colorize for T where T: Display + ?Sized {}
