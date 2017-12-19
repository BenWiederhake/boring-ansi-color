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


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
            &Color::Red => 9,
            &Color::Green => 10,
            &Color::Yellow => 11,
            &Color::Blue => 12,
            &Color::Magenta => 13,
            &Color::Cyan => 14,
            &Color:: White => 15,
        }
    }

    fn is_black(&self) -> bool {
        match self {
            &Color::Black => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Coloring {
    Reset,
    Fg(Color),
    Bg(Color),
    On(bool, Color, bool, Color),
}

impl fmt::Display for Coloring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Coloring::Reset => write!(f, "\x1b[0m"),
            &Coloring::Fg(ref fg) => write!(f, "\x1b[{}3{}m", if fg.is_black() {""} else {"01;"}, fg.to_digit()),
            &Coloring::Bg(ref bg) => write!(f, "\x1b[4{}m", bg.to_digit()),
            &Coloring::On(fg_bright, ref fg, bg_bright, ref bg) => write!(f, "\x1b[38;5;{};48;5;{}m", fg.to_digit(), bg.to_digit()),
        }
    }
}
