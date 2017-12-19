extern crate boring_ansi_color;

use boring_ansi_color::{Color, Coloring};

fn main() {
    let cols = [
        ("Blk_", Color::Black, false),
        ("Red_", Color::Red, false),
        ("Grn_", Color::Green, false),
        ("Ylw_", Color::Yellow, false),
        ("Blu_", Color::Blue, false),
        ("Mgn_", Color::Magenta, false),
        ("Cyn_", Color::Cyan, false),
        ("Wht_", Color::White, false),
        ("Blk+", Color::Black, true),
        ("Red+", Color::Red, true),
        ("Grn+", Color::Green, true),
        ("Ylw+", Color::Yellow, true),
        ("Blu+", Color::Blue, true),
        ("Mgn+", Color::Magenta, true),
        ("Cyn+", Color::Cyan, true),
        ("Wht+", Color::White, true),
    ];
    print!("    ");
    for &(ref fg_name, _, _) in &cols {
        print!(" {}", fg_name);
    }
    println!();
    for &(ref fg_name, ref fg_col, fg_bright) in &cols {
        print!("{}", fg_name);
        for &(_, ref bg_col, bg_bright) in &cols {
            print!(" {}-/\\-{}", Coloring::On(fg_bright, *fg_col, bg_bright, *bg_col), Coloring::Reset);
        }
        println!();
    }
}
