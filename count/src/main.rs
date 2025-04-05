use std::{io, process};

use count::count_lines;

fn main() {
    let lines = count_lines(io::stdin().lock());

    match lines {
        Ok(count) => println!("{count}"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
