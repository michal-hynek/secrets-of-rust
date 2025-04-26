use std::fmt::Display;
use anyhow::{Context, Result};
use reqwest::blocking::RequestBuilder;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct Weather {
    temperature: f64,
    summary: String,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:.1}ºC", self.summary, self.temperature)
    }
}

pub fn get_weather(location: &str, api_key: &str) -> Result<Weather> {
    let resp = request(location, api_key).send()?;
    let weather = deserialize(&resp.text()?)?;
    Ok(weather)
}

fn request(location: &str, api_key: &str) -> RequestBuilder {
    reqwest::blocking::Client::new()
        .get("https://api.weatherstack.com/current")
        .query(&[("query", &location), ("access_key", &api_key)])
}

fn deserialize(json: &str) -> Result<Weather> {
    let val: Value = serde_json::from_str(json)?;
    let temperature = val
        .pointer("/current/temperature")
        .and_then(Value::as_f64)
        .with_context(|| format!("bad response {val}"))?;
    let summary = val
        .pointer("/current/weather_descriptions/0")
        .and_then(Value::as_str)
        .with_context(|| format!("bad response {val}"))?
        .to_string();

    Ok(Weather {
        temperature,
        summary,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use url::Host::Domain;

    use super::*;

    #[test]
    fn request_builds_correct_request() {
        let req = request("Vancouver, BC", "test-api-key");
        let req = req.build().unwrap();

        assert_eq!(req.method(), "GET");
        assert_eq!(req.url().host(), Some(Domain("api.weatherstack.com")));
        assert_eq!(req.url().path(), "/current");

        let params: Vec<(_, _)> = req.url().query_pairs().collect();
        assert_eq!(params, vec![
            ("query".into(), "Vancouver, BC".into()),
            ("access-key".into(), "test-api-key".into()),
        ])
    }

    #[test]
    fn deserialize_returns_weather_struct() {
        let json = fs::read_to_string("tests/data/current_weather.json").unwrap();
        let response = deserialize(&json).unwrap();

        assert_eq!(response, Weather{
            temperature: 11.1,
            summary: "Sunny".into(),
        });
    }

    #[test]
    fn get_weather_returns_correct_weather_for_location() {
        let api_key = "test-1234";
        let location = "Vancouver, BC";
        let weather = get_weather(location, api_key).unwrap();

        assert_eq!(weather, Weather {
            temperature: 11.2,
            summary: "Sunny".into(),
        })
    }
}