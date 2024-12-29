use serde::Deserialize;

use crate::{error::Error, location::Location, Query};

#[derive(Debug, Deserialize)]
struct Response {
  alerts: AlertsResponse,
}

#[derive(Debug, Deserialize)]
struct AlertsResponse {
  alert: Vec<AlertResponse>,
}

#[derive(Debug, Deserialize)]
struct AlertResponse {
  headline: String,
  msgtype: String,
  severity: String,
  urgency: String,
  areas: String,
  category: String,
  certainty: String,
  event: String,
  note: String,
  effective: String,
  expires: String,
  desc: String,
  instruction: String,
}

#[derive(Debug)]
pub struct Alerts {
  alerts: Vec<Alert>,
}

#[derive(Debug)]
pub struct Alert {
  headline: String,
  message_type: String,
  description: String,
  severity: String,
  urgency: String,
  areas: String,
  category: String,
  certainty: String,
  event: String,
  note: String,
  effective: String,
  expires: String,
  instruction: String,
}

impl Alerts {
  pub fn new(alerts: Vec<Alert>) -> Self {
    Alerts { alerts }
  }
}

impl From<Response> for Alerts {
  fn from(response: Response) -> Self {
    let alerts = response.alerts.alert.into_iter().map(Alert::from).collect();
    Alerts::new(alerts)
  }
}

impl ToString for Alerts {
  fn to_string(&self) -> String {
    let mut result = String::new();
    for alert in &self.alerts {
      result.push_str(&format!("{}\n", alert.to_string()));
    }
    result
  }
}

impl Alert {
  pub fn new(
    headline: String,
    message_type: String,
    description: String,
    severity: String,
    urgency: String,
    areas: String,
    category: String,
    certainty: String,
    event: String,
    note: String,
    effective: String,
    expires: String,
    instruction: String,
  ) -> Self {
    Alert {
      headline,
      message_type,
      description,
      severity,
      urgency,
      areas,
      category,
      certainty,
      event,
      note,
      effective,
      expires,
      instruction,
    }
  }
}

impl From<AlertResponse> for Alert {
  fn from(response: AlertResponse) -> Self {
    Alert::new(
      response.headline,
      response.msgtype,
      response.desc,
      response.severity,
      response.urgency,
      response.areas,
      response.category,
      response.certainty,
      response.event,
      response.note,
      response.effective,
      response.expires,
      response.instruction,
    )
  }
}

impl ToString for Alert {
  fn to_string(&self) -> String {
    format!(
      "Headline: {}\nType: {}\nDescription: {}\nSeverity: {}\nUrgency: {}\nAreas: {}\nCategory: {}\nCertainty: {}\nEvent: {}\nNote: {}\nEffective: {}\nExpires: {}\nInstruction: {}\n",
      self.headline,
      self.message_type,
      self.description,
      self.severity,
      self.urgency,
      self.areas,
      self.category,
      self.certainty,
      self.event,
      self.note,
      self.effective,
      self.expires,
      self.instruction
    )
  }
}

/// Get weather alerts for a location.
/// Returns a Result with the Alerts struct or an Error from this crate.
pub async fn get_alerts(location: Location, days: i32) -> Result<Alerts, Error> {
  let location_query = match location.query() {
    Ok(query) => query,
    Err(err) => return Err(err),
  };

  let address = std::env::var("WEATHER_API").unwrap();
  let key = std::env::var("WEATHER_KEY").unwrap();

  let url = format!(
    "{}/forecast.json?key={}&q={}&days={}&alerts=yes",
    address, key, location_query, days
  );

  match reqwest::get(&url).await {
    Ok(response) => {
      let alerts: Response = response.json::<Response>().await.unwrap();
      Ok(Alerts::from(alerts))
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
  async fn test_get_alerts_success() {
    setup();
    let _m = mock(
      "GET",
      "/forecast.json?key=test_key&q=51.5171,-0.1062&days=3&alerts=yes",
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
      r#"{
        "alerts": {
          "alert": [
            {
              "headline": "Severe Weather Alert",
              "msgtype": "Alert",
              "severity": "Severe",
              "urgency": "Immediate",
              "areas": "London",
              "category": "Met",
              "certainty": "Likely",
              "event": "Heavy Rain",
              "note": "Stay indoors",
              "effective": "2024-12-28T10:00:00Z",
              "expires": "2024-12-28T18:00:00Z",
              "desc": "Heavy rain expected in the area.",
              "instruction": "Stay indoors and avoid travel."
            }
          ]
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

    let result = get_alerts(location, 3).await;
    assert!(result.is_ok());

    let alerts = result.unwrap();
    assert_eq!(alerts.alerts.len(), 1);

    let alert = &alerts.alerts[0];
    assert_eq!(alert.headline, "Severe Weather Alert");
    assert_eq!(alert.message_type, "Alert");
    assert_eq!(alert.description, "Heavy rain expected in the area.");
    assert_eq!(alert.severity, "Severe");
    assert_eq!(alert.urgency, "Immediate");
    assert_eq!(alert.areas, "London");
    assert_eq!(alert.category, "Met");
    assert_eq!(alert.certainty, "Likely");
    assert_eq!(alert.event, "Heavy Rain");
    assert_eq!(alert.note, "Stay indoors");
    assert_eq!(alert.effective, "2024-12-28T10:00:00Z");
    assert_eq!(alert.expires, "2024-12-28T18:00:00Z");
    assert_eq!(alert.instruction, "Stay indoors and avoid travel.");
  }

  #[tokio::test]
  async fn test_get_alerts_empty_alerts() {
    setup();
    let _m = mock(
      "GET",
      "/forecast.json?key=test_key&q=51.5171,-0.1062&days=3&alerts=yes",
    )
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(
      r#"{
        "alerts": {
          "alert": []
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

    let result = get_alerts(location, 3).await;
    assert!(result.is_ok());

    let alerts = result.unwrap();
    assert_eq!(alerts.alerts.len(), 0);
  }

  #[tokio::test]
  async fn test_get_alerts_no_location() {
    setup();

    let location = Location {
      country: "United Kingdom".to_string(),
      region: Some("City of London, Greater London".to_string()),
      city: Some("London".to_string()),
      lat: None,
      lon: None,
    };

    let result = get_alerts(location, 3).await;
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error::NoLocation);
  }
}
