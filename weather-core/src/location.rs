use crate::{
  error::Error,
  ip::{self},
  Query,
};

#[derive(Debug, Clone)]
pub struct Location {
  pub country: String,
  pub region: Option<String>,
  pub city: Option<String>,
  pub lat: Option<f64>,
  pub lon: Option<f64>,
}

impl Query<String> for Location {
  /// Generate the query result for a location.
  /// If the location does not have a latitude and longitude, an error is returned.
  /// Otherwise {lat},{lon} pairs are returned as a string.
  fn query(&self) -> Result<String, Error> {
    if self.lat.is_none() && self.lon.is_none() {
      return Err(Error::NoLocation);
    }

    let lat = self.lat.unwrap();
    let lon = self.lon.unwrap();

    Ok(format!("{},{}", lat, lon))
  }
}

impl ToString for Location {
  fn to_string(&self) -> String {
    match &self.region {
      Some(region) => format!(
        "{}, {}, {}",
        self.city.as_ref().unwrap(),
        region,
        self.country
      ),
      None => format!("{}, {}", self.city.as_ref().unwrap(), self.country),
    }
  }
}

pub async fn get_location_from_ip(ip: &str) -> Result<Location, Error> {
  match ip::get_location(ip.to_string()).await {
    Ok(location) => Ok(Location {
      country: location.country,
      region: Some(location.region),
      city: Some(location.city),
      lat: Some(location.lat),
      lon: Some(location.lon),
    }),
    Err(err) => Err(Error::BadIp {
      ip: ip.to_string(),
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
    std::env::set_var("IP_LOCATION_API", &mockito::server_url());
    std::env::set_var("PUBLIC_IP_API", &mockito::server_url());
  }

  #[test]
  fn test_query_success() {
    let location = Location {
      country: "United Kingdom".to_string(),
      region: Some("City of London, Greater London".to_string()),
      city: Some("London".to_string()),
      lat: Some(51.5171),
      lon: Some(-0.1062),
    };

    let result = location.query();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "51.5171,-0.1062");
  }

  #[test]
  fn test_query_no_lat_lon() {
    let location = Location {
      country: "United Kingdom".to_string(),
      region: Some("City of London, Greater London".to_string()),
      city: Some("London".to_string()),
      lat: None,
      lon: None,
    };

    let result = location.query();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error::NoLocation);
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

    let result = get_location_from_ip("8.8.8.8").await;
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
    let result = get_location_from_ip("2.2.2.2").await;
    assert!(result.is_err());
  }
}
