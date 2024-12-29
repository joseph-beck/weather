use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Response {
  pub status: Status,
  pub country: String,
  #[serde(rename = "countryCode")]
  pub country_code: String,
  pub region: String,
  #[serde(rename = "regionName")]
  pub region_name: String,
  pub city: String,
  pub zip: String,
  pub lat: f64,
  pub lon: f64,
  pub isp: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Status {
  #[serde(rename = "success")]
  Success,
  #[serde(rename = "fail")]
  Fail,
}

pub struct IP {
  pub status: Status,
  pub country: String,
  pub country_code: String,
  pub region: String,
  pub region_name: String,
  pub city: String,
  pub zip: String,
  pub lat: f64,
  pub lon: f64,
  pub isp: String,
}

impl IP {
  pub fn new(
    status: Status,
    country: String,
    country_code: String,
    region: String,
    region_name: String,
    city: String,
    zip: String,
    lat: f64,
    lon: f64,
    isp: String,
  ) -> Self {
    IP {
      status,
      country,
      country_code,
      region,
      region_name,
      city,
      zip,
      lat,
      lon,
      isp,
    }
  }
}

impl From<Response> for IP {
  fn from(response: Response) -> Self {
    IP::new(
      response.status,
      response.country,
      response.country_code,
      response.region,
      response.region_name,
      response.city,
      response.zip,
      response.lat,
      response.lon,
      response.isp,
    )
  }
}

/// Get the location from a given IP address.
/// IP given as string.
/// Returns a Result with the IP struct or a reqwest error.
pub async fn get_location(ip: String) -> Result<IP, reqwest::Error> {
  let address = env::var("IP_LOCATION_API").unwrap();
  let url = format!("{}/{}", address, ip);
  let response = reqwest::get(&url).await?;
  let location: Response = response.json::<Response>().await?;
  Ok(IP::from(location))
}

/// Get the public IP address of the current machine.
/// Returns a Result with the IP address or a reqwest error.
pub async fn get_public_ip() -> Result<String, reqwest::Error> {
  #[derive(Deserialize)]
  struct Response {
    ip: String,
  }

  let address = env::var("PUBLIC_IP_API").unwrap();
  let response = reqwest::get(address).await?;
  let ip: Response = response.json().await?;
  Ok(ip.ip)
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito::mock;
  use tokio;

  fn setup() {
    dotenv::dotenv().ok();
    std::env::set_var("IP_LOCATION_API", &mockito::server_url());
    std::env::set_var("PUBLIC_IP_API", &mockito::server_url());
  }

  #[tokio::test]
  async fn test_get_location_success() {
    setup();
    let _m = mock("GET", "/8.8.8.8")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(
        r#"{
          "query": "8.8.8.8",
          "status": "success",
          "country": "United States",
          "countryCode": "US",
          "region": "VA",
          "regionName": "Virginia",
          "city": "Ashburn",
          "zip": "20149",
          "lat": 39.03,
          "lon": -77.5,
          "timezone": "America/New_York",
          "isp": "Google LLC",
          "org": "Google Public DNS",
          "as": "AS15169 Google LLC"
        }"#,
      )
      .create();

    let result = get_location("8.8.8.8".to_string()).await;
    assert!(result.is_ok());

    let location = result.unwrap();
    assert_eq!(location.status, Status::Success);
    assert_eq!(location.country, "United States");
    assert_eq!(location.country_code, "US");
    assert_eq!(location.region, "VA");
    assert_eq!(location.region_name, "Virginia");
    assert_eq!(location.city, "Ashburn");
    assert_eq!(location.zip, "20149");
    assert_eq!(location.lat, 39.03);
    assert_eq!(location.lon, -77.5);
    assert_eq!(location.isp, "Google LLC");
  }

  #[tokio::test]
  async fn test_get_location_fail() {
    setup();
    let _m = mock("GET", "/1")
      .with_status(500)
      .with_header("content-type", "application/json")
      .with_body(
        r#"{
          "query": "1",
          "status": "fail",
          "country": "",
          "countryCode": "",
          "region": "",
          "regionName": "",
          "city": "",
          "zip": "",
          "lat": 0.0,
          "lon": 0.0,
          "isp": ""
        }"#,
      )
      .create();

    let result = get_location("1".to_string()).await;
    assert!(result.is_ok());

    let location = result.unwrap();
    assert_eq!(location.status, Status::Fail);
  }

  #[tokio::test]
  async fn test_get_public_ip_success() {
    setup();
    let _m = mock("GET", "/")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(r#"{ "ip": "8.8.8.8" }"#)
      .create();

    let result = get_public_ip().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "8.8.8.8");
  }

  #[tokio::test]
  async fn test_get_public_ip_fail() {
    setup();
    let _m = mock("GET", "/")
      .with_status(500)
      .with_header("content-type", "application/json")
      .with_body(r#"{ "error": "Internal Server Error" }"#)
      .create();

    let result = get_public_ip().await;
    assert!(result.is_err());
  }
}
