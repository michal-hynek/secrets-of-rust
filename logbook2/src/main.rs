use std::env;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let path = "logbook.txt";
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        match logbook::read(path)? {
            Some(content) => print!("{content}"),
            None => println!("Logbook is empty"),
        }
    } else {
        let message = args.join(" ");
        logbook::append(path, &message)?;
    }

    Ok(())
}
