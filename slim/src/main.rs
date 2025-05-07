use anyhow::{Ok, Result};
use clap::Parser;

use slim::Slimmer;

#[derive(Parser)]
/// Runs `cargo clean` recursively to save disk space by deleting build artifacts.
struct Args {
    #[arg(default_value = ".")]
    /// Path to search for Rust projects
    path: String,

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

    let output = slimmer.slim(args.path)?;
    print!("{output}");

    Ok(())
}