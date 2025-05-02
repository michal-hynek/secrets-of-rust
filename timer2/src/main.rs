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

    let report = timer::time(&args.program, &args.args)?;

    print!("stdout: {}", report.stdout);
    println!("stderr: {}", report.stderr);
    println!("command took: {:.1?}", report.elapsed);

    Ok(())
}
