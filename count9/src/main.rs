use count::count_in_path;

use clap::Parser;

#[derive(Parser)]
/// Counts lines or words in the specified files
struct Args {
    /// Counts words instead of lines
    #[arg(short, long)]
    words: bool,

    /// Files to be counted
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for path in args.files {
        if args.words {
            println!("{path}: {} words", count_in_path(&path)?.words);
        } else {
            println!("{path}: {} lines", count_in_path(&path)?.lines);
        }
    }

    Ok(())
}
