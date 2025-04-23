use std::env;
use clap::Parser;
use anyhow::Result;

use weather::get_weather;

#[derive(Parser)]
struct Args {
    /// WeatherStack API key
    #[arg(short, long)]
    api_key: String,

    /// Location
    #[arg(required = true)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let api_key = env::var("WEATHERSTACK_API_KEY").unwrap();
    let location = args.args.join(" ");

    let weather = get_weather(&location, &api_key)?;
    println!("{weather:?}");

    Ok(())
}
