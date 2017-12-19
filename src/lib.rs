use std::fmt;

/*
 * MIT License
 *
 * Copyright (c) 2017 Ben Wiederhake
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
*/


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
    Fg(Color),
    Bg(Color),
    Of(Color, Color),
}

impl fmt::Display for Coloring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Coloring::Reset => write!(f, "\x1b[0m"),
            &Coloring::Fg(ref fg) => write!(f, "\x1b[3{}m", fg.to_digit()),
            &Coloring::Bg(ref bg) => write!(f, "\x1b[4{}m", bg.to_digit()),
            &Coloring::Of(ref fg, ref bg) => write!(f, "\x1b[3{};4{}m", fg.to_digit(), bg.to_digit()),
        }
    }
}
