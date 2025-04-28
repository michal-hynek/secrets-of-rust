use clap::Parser;
use anyhow::Result;
use weather::Weatherstack;

#[derive(Parser)]
struct Args {
    #[arg(short, long, env = "WEATHERSTACK_API_KEY", required = true)]
    /// WeatherStack API key
    api_key: String,

    #[arg(short, long)]
    /// Reports temperatures in Fahrenheit
    fahrenheit: bool,

    #[arg(required = true)]
    /// Location
    /// Example: "Vancouver, BC"
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let api_key = args.api_key;
    let location = args.args.join(" ");
    let ws = Weatherstack::new(&api_key);

    let weather = ws.get_weather(&location)?;

    if args.fahrenheit {
        println!("{}", weather.into_fahrenheit());
    } else {
        println!("{weather}");
    }

    Ok(())
}
