use std::env;

use anyhow::anyhow;
use count::count_in_path;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow!("Usage: count [-w] <FILE>..."));
    }

    let mut count_words = false;

    for path in env::args().skip(1) {
        if path == "-w" {
            count_words = true;
            continue;
        }

        if count_words {
            println!("{path}: {} words", count_in_path(&path)?.words);
        } else {
            println!("{path}: {} lines", count_in_path(&path)?.lines);
        }
    }

    Ok(())
}
