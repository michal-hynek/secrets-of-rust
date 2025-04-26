use clap::Parser;
use anyhow::Result;

use weather::get_weather;

#[derive(Parser)]
struct Args {
    /// WeatherStack API key
    #[arg(short, long, env = "WEATHERSTACK_API_KEY", required = true)]
    api_key: String,

    /// Location
    #[arg(required = true)]
    /// Example: "Vancouver, BC"
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let api_key = args.api_key;
    let location = args.args.join(" ");

    let weather = get_weather(&location, &api_key)?;
    println!("{weather}");

    Ok(())
}
