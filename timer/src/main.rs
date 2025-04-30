use std::process::Command;
use std::time::Instant;

use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    let start = Instant::now();

    let mut cmd = Command::new("cargo");
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("command output: {stdout}");
    println!("command took: {:?}", start.elapsed());

    Ok(())
}
