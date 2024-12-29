use chrono::Local;
use serde::Deserialize;

use crate::{error::Error, location::Location};

#[derive(Deserialize, Debug)]
struct Response {
  astronomy: AstronomyResponse,
}

#[derive(Deserialize, Debug)]
struct AstronomyResponse {
  astro: AstroResponse,
}

#[derive(Deserialize, Debug)]
struct AstroResponse {
  sunrise: String,
  sunset: String,
  moonrise: String,
  moonset: String,
  moon_phase: String,
  moon_illumination: i32,
  is_moon_up: i32,
  is_sun_up: i32,
}

#[derive(Debug)]
pub struct Astronomy {
  pub sunrise: String,
  pub sunset: String,
  pub moonrise: String,
  pub moonset: String,
  pub moon_phase: String,
  pub moon_illumination: i32,
  pub is_moon_up: bool,
  pub is_sun_up: bool,
}

impl Astronomy {
  pub fn new(
    sunrise: String,
    sunset: String,
    moonrise: String,
    moonset: String,
    moon_phase: String,
    moon_illumination: i32,
    is_moon_up: i32,
    is_sun_up: i32,
  ) -> Self {
    Astronomy {
      sunrise,
      sunset,
      moonrise,
      moonset,
      moon_phase,
      moon_illumination,
      is_moon_up: is_moon_up == 1,
      is_sun_up: is_sun_up == 1,
    }
  }
}

impl From<Response> for Astronomy {
  fn from(response: Response) -> Self {
    Astronomy::new(
      response.astronomy.astro.sunrise,
      response.astronomy.astro.sunset,
      response.astronomy.astro.moonrise,
      response.astronomy.astro.moonset,
      response.astronomy.astro.moon_phase,
      response.astronomy.astro.moon_illumination,
      response.astronomy.astro.is_moon_up,
      response.astronomy.astro.is_sun_up,
    )
  }
}

/// Get the current astronomy data for a given location.
/// By default uses the lat and lon data from the location struct, if it exists.
/// If there is no location data within the struct, it will return an error.
pub async fn get_current_astronomy(location: Location) -> Result<Astronomy, Error> {
  if location.lat.is_none() && location.lon.is_none() {
    return Err(Error::NoLocation);
  }

  let lat = location.lat.unwrap();
  let lon = location.lon.unwrap();
  let dt = Local::now().format("%Y-%m-%d").to_string();

  let address = std::env::var("WEATHER_API").unwrap();
  let key = std::env::var("WEATHER_KEY").unwrap();

  let url = format!(
    "{}/astronomy.json?key={}&q={},{}&dt={}",
    address, key, lat, lon, dt
  );

  match reqwest::get(&url).await {
    Ok(response) => {
      let weather: Response = response.json().await.unwrap();
      Ok(Astronomy::from(weather))
    }
    Err(err) => Err(Error::Fetch {
      message: err.to_string(),
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito::mock;
  use tokio;

  fn setup() {
    dotenv::dotenv().ok();
    std::env::set_var("WEATHER_API", &mockito::server_url());
    std::env::set_var("WEATHER_KEY", "test_key");
  }

  #[tokio::test]
  async fn test_get_current_astronomy_success() {
    setup();
    let _m = mock(
      "GET",
      format!(
        "/astronomy.json?key=test_key&q=51.5171,-0.1062&dt={}",
        Local::now().format("%Y-%m-%d")
      )
      .as_str(),
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
      r#"{
        "location": {
          "name": "London",
          "region": "City of London, Greater London",
          "country": "United Kingdom",
          "lat": 51.5171,
          "lon": -0.1062,
          "tz_id": "Europe/London",
          "localtime": "2024-12-28 11:39"
        },
        "astronomy": {
          "astro": {
            "sunrise": "08:06 AM",
            "sunset": "03:58 PM",
            "moonrise": "06:11 AM",
            "moonset": "01:21 PM",
            "moon_phase": "Waning Crescent",
            "moon_illumination": 9,
            "is_moon_up": 0,
            "is_sun_up": 0
          }
        }
      }"#,
    )
    .create();

    let location = Location {
      country: "United Kingdom".to_string(),
      region: Some("City of London, Greater London".to_string()),
      city: Some("London".to_string()),
      lat: Some(51.5171),
      lon: Some(-0.1062),
    };

    let result = get_current_astronomy(location).await;
    assert!(result.is_ok());

    let astronomy = result.unwrap();
    assert_eq!(astronomy.sunrise, "08:06 AM");
    assert_eq!(astronomy.sunset, "03:58 PM");
    assert_eq!(astronomy.moon_phase, "Waning Crescent");
    assert_eq!(astronomy.moon_illumination, 9);
    assert_eq!(astronomy.is_moon_up, false);
    assert_eq!(astronomy.is_sun_up, false);
  }

  #[tokio::test]
  async fn test_get_current_astronomy_no_location() {
    setup();

    let location = Location {
      country: "United Kingdom".to_string(),
      region: Some("City of London, Greater London".to_string()),
      city: Some("London".to_string()),
      lat: None,
      lon: None,
    };

    let result = get_current_astronomy(location).await;
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error::NoLocation);
  }
}
