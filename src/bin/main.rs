extern crate boring_ansi_color;

use boring_ansi_color::{Color, Coloring};

fn main() {
    println!("Hello, {}World{}!", Coloring::Of(Color::Red, Color::Magenta), Coloring::Reset);
}
