use std::io::BufReader;
use std::{env, process};
use std::fs::File;

use count::count_lines;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Usage count <FILE>");
        process::exit(1);
    };

    let file = File::open(&path)
        .map_err(|e| {
            eprintln!("{e}");
            process::exit(1);
        })
        .unwrap();
    let file = BufReader::new(file);

    let lines = count_lines(file)
        .map_err(|e| {
            eprintln!("{e}");
            process::exit(1);
        })
        .unwrap();

    println!("{lines}");
}
