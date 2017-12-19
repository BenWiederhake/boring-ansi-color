use std::fmt;

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    fn to_digit(&self) -> u8 {
        match self {
            &Color::Black => 0,
            &Color::Red => 1,
            &Color::Green => 2,
            &Color::Yellow => 3,
            &Color::Blue => 4,
            &Color::Magenta => 5,
            &Color::Cyan => 6,
            &Color:: White => 7,
        }
    }
}

pub enum Coloring {
    Reset,
    Of(Color, Color),
}

impl fmt::Display for Coloring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Coloring::Reset => write!(f, "\x1b[0m"),
            &Coloring::Of(ref fg, ref bg) => write!(f, "\x1b[3{};4{}m", fg.to_digit(), bg.to_digit()),
        }
    }
}
