use std::process::Command;
use std::time::Instant;

use clap::Parser;

use anyhow::Ok;

#[derive(Parser)]
struct Args {
    /// Program to run
    #[arg(required = true)]
    program: String,

    /// Program arguments
    args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let start = Instant::now();
    let mut cmd = Command::new(args.program);
    cmd.args(args.args);
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr= String::from_utf8_lossy(&output.stderr);

    print!("stdout: {stdout}");
    println!("stderr: {stderr}");
    println!("command took: {:.1?}", start.elapsed());

    Ok(())
}
