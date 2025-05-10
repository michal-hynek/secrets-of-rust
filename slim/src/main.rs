use anyhow::{Ok, Result};
use clap::Parser;

use slim::Slimmer;

#[derive(Parser)]
/// Runs `cargo clean` recursively to save disk space by deleting build artifacts.
struct Args {
    #[arg(default_value = ".")]
    /// Paths to search for Rust projects
    paths: Vec<String>,

    #[arg(long)]
    /// Dry run without deleting any files
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut slimmer = Slimmer::new();

    if args.dry_run {
        slimmer.dry_run = true;
    }

    for path in args.paths {
        let output = slimmer.slim(path)?;
        print!("{output}");
    }

    Ok(())
}