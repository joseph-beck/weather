use serde::Deserialize;

use crate::{error::Error, location::Location, FromResponse, FromResponseWithOption};

#[derive(Debug, Deserialize)]
struct Response {
  location: LocationResponse,
  current: CurrentResponse,
}

#[derive(Debug, Deserialize)]
struct LocationResponse {
  name: String,
  region: String,
  country: String,
  lat: f64,
  lon: f64,
  tz_id: String,
  localtime: String,
}

#[derive(Debug, Deserialize)]
struct CurrentResponse {
  temp_c: f64,
  temp_f: f64,
  is_day: i32,
  condition: ConditionResponse,
  wind_mph: f64,
  wind_kph: f64,
  wind_degree: i32,
  wind_dir: String,
  pressure_mb: f64,
  pressure_in: f64,
  precip_mm: f64,
  precip_in: f64,
  humidity: i32,
  cloud: i32,
  feelslike_c: f64,
  feelslike_f: f64,
  windchill_c: f64,
  windchill_f: f64,
  heatindex_c: f64,
  heatindex_f: f64,
  dewpoint_c: f64,
  dewpoint_f: f64,
  vis_km: f64,
  vis_miles: f64,
  uv: f64,
  gust_mph: f64,
  gust_kph: f64,
}

#[derive(Debug, Deserialize)]
struct ConditionResponse {
  text: String,
  icon: String,
  code: i32,
}

#[derive(Debug, PartialEq)]
pub enum Units {
  Metric,
  Imperial,
}

#[derive(Debug)]
pub struct Weather {
  pub is_day: bool,
  pub temperature: f64,
  pub feels_like: f64,
  pub heat_index: f64,
  pub condition: Condition,
  pub wind_speed: f64,
  pub wind_degree: i32,
  pub wind_dir: String,
  pub wind_gust_speed: f64,
  pub wind_chill: f64,
  pub pressure: f64,
  pub precipitation: f64,
  pub humidity: i32,
  pub cloud: i32,
  pub uv: f64,
  pub visibility: f64,
  pub dew_point: f64,
}

#[derive(Debug)]
pub struct Condition {
  pub text: String,
  pub icon: String,
  pub code: i32,
}

impl Weather {
  pub fn new(
    is_day: bool,
    temperature: f64,
    feels_like: f64,
    heat_index: f64,
    condition: Condition,
    wind_speed: f64,
    wind_degree: i32,
    wind_dir: String,
    wind_gust_speed: f64,
    wind_chill: f64,
    pressure: f64,
    precipitation: f64,
    humidity: i32,
    cloud: i32,
    uv: f64,
    visibility: f64,
    dew_point: f64,
  ) -> Self {
    Weather {
      is_day,
      temperature,
      feels_like,
      heat_index,
      condition,
      wind_speed,
      wind_degree,
      wind_dir,
      wind_gust_speed,
      wind_chill,
      pressure,
      precipitation,
      humidity,
      cloud,
      uv,
      visibility,
      dew_point,
    }
  }
}

impl FromResponseWithOption<Response, Units> for Weather {
  fn new_from_response_with_options(response: Response, units: Units) -> Self {
    if units == Units::Imperial {
      return Weather::new(
        response.current.is_day == 1,
        response.current.temp_f,
        response.current.feelslike_f,
        response.current.heatindex_f,
        Condition::new_from_response(response.current.condition),
        response.current.wind_mph,
        response.current.wind_degree,
        response.current.wind_dir,
        response.current.gust_mph,
        response.current.windchill_f,
        response.current.pressure_in,
        response.current.precip_in,
        response.current.humidity,
        response.current.cloud,
        response.current.uv,
        response.current.vis_miles,
        response.current.dewpoint_f,
      );
    } else {
      return Weather::new(
        response.current.is_day == 1,
        response.current.temp_c,
        response.current.feelslike_c,
        response.current.heatindex_c,
        Condition::new_from_response(response.current.condition),
        response.current.wind_kph,
        response.current.wind_degree,
        response.current.wind_dir,
        response.current.gust_kph,
        response.current.windchill_c,
        response.current.pressure_mb,
        response.current.precip_mm,
        response.current.humidity,
        response.current.cloud,
        response.current.uv,
        response.current.vis_km,
        response.current.dewpoint_c,
      );
    }
  }
}

impl Condition {
  pub fn new(text: String, icon: String, code: i32) -> Self {
    Condition { text, icon, code }
  }
}

impl FromResponse<ConditionResponse> for Condition {
  fn new_from_response(response: ConditionResponse) -> Self {
    Condition::new(response.text, response.icon, response.code)
  }
}

pub async fn get_current_weather(location: Location, units: Units) -> Result<Weather, Error> {
  if location.lat.is_none() && location.lon.is_none() {
    return Err(Error::NoLocation);
  }

  let lat = location.lat.unwrap();
  let lon = location.lon.unwrap();

  let address = std::env::var("WEATHER_API").unwrap();
  let key = std::env::var("WEATHER_KEY").unwrap();

  let url = format!("{}/current.json?key={}&q={},{}", address, key, lat, lon);

  match reqwest::get(&url).await {
    Ok(response) => {
      let weather: Response = response.json().await.unwrap();
      Ok(Weather::new_from_response_with_options(weather, units))
    }
    Err(err) => Err(Error::Fetch {
      message: err.to_string(),
    }),
  }
}

pub async fn get_forecast_weather(
  location: Location,
  units: Units,
  days: i32,
) -> Result<Weather, Error> {
  if location.lat.is_none() && location.lon.is_none() {
    return Err(Error::NoLocation);
  }

  let lat = location.lat.unwrap();
  let lon = location.lon.unwrap();

  let address = std::env::var("WEATHER_API").unwrap();
  let key = std::env::var("WEATHER_KEY").unwrap();

  let url = format!(
    "{}/forecast.json?key={}&q={},{}&days={}",
    address, key, lat, lon, days
  );

  match reqwest::get(&url).await {
    Ok(response) => {
      let weather: Response = response.json().await.unwrap();
      Ok(Weather::new_from_response_with_options(weather, units))
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

  // I couldn't be bothered to write these test so AI did :)
  #[tokio::test]
  async fn test_get_current_weather_metric() {
    setup();
    let _m = mock("GET", "/current.json?key=test_key&q=51.5171,-0.1062")
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
            "localtime": "2024-12-28 00:51"
          },
          "current": {
            "temp_c": 5.1,
            "temp_f": 41.2,
            "is_day": 0,
            "condition": {
              "text": "Fog",
              "icon": "//cdn.weatherapi.com/weather/64x64/night/248.png",
              "code": 1135
            },
            "wind_mph": 2.2,
            "wind_kph": 3.6,
            "wind_degree": 206,
            "wind_dir": "SSW",
            "pressure_mb": 1030,
            "pressure_in": 30.42,
            "precip_mm": 0,
            "precip_in": 0,
            "humidity": 100,
            "cloud": 100,
            "feelslike_c": 4.8,
            "feelslike_f": 40.7,
            "windchill_c": 5.1,
            "windchill_f": 41.2,
            "heatindex_c": 5.1,
            "heatindex_f": 41.2,
            "dewpoint_c": 4.4,
            "dewpoint_f": 39.9,
            "vis_km": 0.4,
            "vis_miles": 0,
            "uv": 0,
            "gust_mph": 2.5,
            "gust_kph": 4.1
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

    let result = get_current_weather(location, Units::Metric).await;
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 5.1);
    assert_eq!(weather.feels_like, 4.8);
    assert_eq!(weather.condition.text, "Fog");
  }

  #[tokio::test]
  async fn test_get_current_weather_imperial() {
    setup();
    let _m = mock("GET", "/current.json?key=test_key&q=51.5171,-0.1062")
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
            "localtime": "2024-12-28 00:51"
          },
          "current": {
            "temp_c": 5.1,
            "temp_f": 41.2,
            "is_day": 0,
            "condition": {
              "text": "Fog",
              "icon": "//cdn.weatherapi.com/weather/64x64/night/248.png",
              "code": 1135
            },
            "wind_mph": 2.2,
            "wind_kph": 3.6,
            "wind_degree": 206,
            "wind_dir": "SSW",
            "pressure_mb": 1030,
            "pressure_in": 30.42,
            "precip_mm": 0,
            "precip_in": 0,
            "humidity": 100,
            "cloud": 100,
            "feelslike_c": 4.8,
            "feelslike_f": 40.7,
            "windchill_c": 5.1,
            "windchill_f": 41.2,
            "heatindex_c": 5.1,
            "heatindex_f": 41.2,
            "dewpoint_c": 4.4,
            "dewpoint_f": 39.9,
            "vis_km": 0.4,
            "vis_miles": 0,
            "uv": 0,
            "gust_mph": 2.5,
            "gust_kph": 4.1
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

    let result = get_current_weather(location, Units::Imperial).await;
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 41.2);
    assert_eq!(weather.feels_like, 40.7);
    assert_eq!(weather.condition.text, "Fog");
  }

  #[tokio::test]
  async fn test_get_current_weather_no_location() {
    setup();

    let location = Location {
      country: "United Kingdom".to_string(),
      region: Some("City of London, Greater London".to_string()),
      city: Some("London".to_string()),
      lat: None,
      lon: None,
    };

    let result = get_current_weather(location, Units::Metric).await;
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error::NoLocation);
  }
}
