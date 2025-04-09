use std::env;

use anyhow::anyhow;
use count::count_lines_in_path;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow!("Usage: count <FILE>..."));
    }

    for path in env::args().skip(1) {
        println!("{path}: {}", count_lines_in_path(&path)?);
    }

    Ok(())
}
