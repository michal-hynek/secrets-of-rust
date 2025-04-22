use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("no location specified");
        process::exit(1);
    }

    let api_key = env::var("WEATHERSTACK_API_KEY").unwrap();
    let location = args.join(" ");
    let resp = reqwest::blocking::Client::new()
        .get("https://api.weatherstack.com/current")
        .query(&[("query", &location), ("access_key", &api_key)])
        .send()
        .unwrap();

    println!("{}", resp.text().unwrap());
}
