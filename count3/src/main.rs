use std::io::BufReader;
use std::env;
use std::fs::File;

use anyhow::Context;
use count::count_lines;

fn main() -> anyhow::Result<()> {
    let path = env::args()
        .nth(1)
        .context("Usage: count <FILE>")?;
    
    let file = File::open(&path)?;
    let file = BufReader::new(file);
    let lines = count_lines(file)?;

    println!("{lines}");

    Ok(())
}
