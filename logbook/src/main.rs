use std::{env, fs::{self, File}, io::Write};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let path = "logbook.txt";
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        if fs::exists("logbook.txt")? {
            let content = fs::read_to_string(path)?;
            print!("{}", content);
        } else {
            println!("Logbook is empty");
        }
    } else {
        let mut logbook = File::options()
            .create(true)
            .append(true)
            .open(path)?;

        let message = args.join(" ");
        writeln!(logbook, "{}", message)?;
    }

    Ok(())
}
