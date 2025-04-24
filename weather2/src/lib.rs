use std::env;
use anyhow::Result;
use reqwest::blocking::RequestBuilder;

#[derive(Debug, PartialEq)]
pub struct Weather {
    temperature: f64,
    summary: String,
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

mod tests {
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
            ("query".into(), "test-api-key".into()),
        ])
    }

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