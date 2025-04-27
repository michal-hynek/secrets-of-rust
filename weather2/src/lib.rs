use std::fmt::Display;
use anyhow::{Context, Result};
use reqwest::blocking::RequestBuilder;
use serde_json::Value;

pub struct Weatherstack {
    pub base_url: String,
    api_key: String,
}

impl Weatherstack {
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: "https://api.weatherstack.com/current".into(),
            api_key: api_key.to_owned(),
        }
    }

    pub fn get_weather(&self, location: &str) -> Result<Weather> {
        let resp = request(&self.base_url, location, &self.api_key).send()?;
        let weather = deserialize(&resp.text()?)?;
        Ok(weather)
    }
}

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

fn request(url: &str, location: &str, api_key: &str) -> RequestBuilder {
    reqwest::blocking::Client::new()
        .get(url)
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
    use httpmock::{Method, MockServer};
    use reqwest::StatusCode;
    use url::Host::Domain;

    use super::*;

    #[test]
    fn request_builds_correct_request() {
        let req = request("https://api.weatherstack.com/current", "Vancouver, BC", "test-api-key");
        let req = req.build().unwrap();

        assert_eq!(req.method(), "GET");
        assert_eq!(req.url().host(), Some(Domain("api.weatherstack.com")));
        assert_eq!(req.url().path(), "/current");

        let params: Vec<(_, _)> = req.url().query_pairs().collect();
        assert_eq!(params, vec![
            ("query".into(), "Vancouver, BC".into()),
            ("access_key".into(), "test-api-key".into()),
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
    fn get_weather_returns_correct_weather() {
        let api_key = "test-key";
        let location = "Vancouver,BC";

        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(Method::GET)
                .path("/current")
                .query_param("query", location)
                .query_param("access_key", api_key);
            then.status(StatusCode::OK.into())
                .header("content-type", "application/json")
                .body_from_file("tests/data/current_weather.json");
        });

        let mut ws = Weatherstack::new(api_key);
        ws.base_url = server.base_url() + "/current";
        let weather = ws.get_weather(location);

        mock.assert();

        assert_eq!(weather.unwrap(), Weather {
            summary: "Sunny".into(),
            temperature: 11.1,
        });
    }
}