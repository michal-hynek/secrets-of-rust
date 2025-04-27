use clap::Parser;
use anyhow::Result;
use weather::Weatherstack;

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
    let ws = Weatherstack::new(&api_key);

    let weather = ws.get_weather(&location)?;
    println!("{weather}");

    Ok(())
}
