use crate::{
  error,
  ip::{self},
};

#[derive(Debug)]
pub struct Location {
  country: String,
  region: Option<String>,
  city: Option<String>,
  lat: Option<f64>,
  lon: Option<f64>,
}

impl Location {
  pub async fn from_ip(ip: &str) -> Result<Location, error::Error> {
    match ip::get_location(ip.to_string()).await {
      Ok(location) => Ok(Location {
        country: location.country,
        region: Some(location.region),
        city: Some(location.city),
        lat: Some(location.lat),
        lon: Some(location.lon),
      }),
      Err(err) => Err(error::Error::BadIp {
        ip: ip.to_string(),
        message: err.to_string(),
      }),
    }
  }
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
  async fn test_from_ip_success() {
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
          "isp": "Google LLC"
        }"#,
      )
      .create();

    let result = Location::from_ip("8.8.8.8").await;
    assert!(result.is_ok());

    let location = result.unwrap();
    assert_eq!(location.country, "United States");
    assert_eq!(location.region, Some("VA".to_string()));
    assert_eq!(location.city, Some("Ashburn".to_string()));
    assert_eq!(location.lat, Some(39.03));
    assert_eq!(location.lon, Some(-77.5));
  }

  #[tokio::test]
  async fn test_from_ip_fail() {
    setup();
    let _m = mock("GET", "/1.1.1.1")
      .with_status(500)
      .with_header("content-type", "application/json")
      .with_body(r#"{ "" }"#)
      .create();

    // The ip is intentionally mismatched.
    let result = Location::from_ip("2.2.2.2").await;
    assert!(result.is_err());
  }
}
