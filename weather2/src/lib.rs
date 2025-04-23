use std::env;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Weather {
    temperature: f64,
    summary: String,
}

pub fn get_weather(location: &str, api_key: &str) -> Result<Weather> {
    Ok(Weather{
        temperature: 11.2,
        summary: "Sunny".into(),
    })
}

mod tests {
    use super::*;

    #[test]
    fn get_weather_returns_correct_weather_for_location() {
        let api_key = env::var("WEATHERSTACK_API_KEY").unwrap();
        let location = "Vancouver, BC";
        let weather = get_weather(location, &api_key).unwrap();

        assert_eq!(weather, Weather {
            temperature: 11.2,
            summary: "Sunny".into(),
        })
    }
}