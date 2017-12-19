# boring-ansi-color

> The most boring ANSI coloror.

I needed to color something.  All the other libraries implement fancy wrappers and pull large dependencies.

This one just does the job: It gives you colors that can be output.

Example:

```Rust
extern crate boring_ansi_color;

use boring_ansi_color::{Color, Coloring};

fn main() {
    println!("Hello, {}World{}!", Coloring::Of(Color::Red, Color::Magenta), Coloring::Reset);
}
```

## TODO

I'm very reluctant to add *anything*.  So I'm probably not going to do anything further.
It does the job, and that's it.

## LICENSE

MIT.  Do whatever you want with it.

Acknowledging me as the author would be nice.
