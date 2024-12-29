use weather_core::{
  alert::get_alerts,
  astronomy::get_current_astronomy,
  error::Error,
  ip::get_public_ip,
  location::get_location_from_ip,
  weather::{get_current_weather, get_forecast_weather, Units},
};

use crate::validate::validate_days;

pub async fn handle_alerts(days: i32) -> Result<(), Error> {
  println!("Weather alerts");

  let days_valid = validate_days(days);
  if days_valid.is_err() {
    return days_valid;
  }

  match get_public_ip().await {
    Ok(ip) => {
      let location = get_location_from_ip(&ip).await.unwrap();
      println!("{:?}", location.to_string());

      let alerts = get_alerts(location.clone(), days).await.unwrap();
      println!("{:?}", alerts.to_string());

      Ok(())
    }
    Err(err) => Err(Error::Error {
      message: err.to_string(),
    }),
  }
}

pub async fn handle_astronomy() -> Result<(), Error> {
  println!("Current astronomy");
  match get_public_ip().await {
    Ok(ip) => {
      let location = get_location_from_ip(&ip).await.unwrap();
      println!("{:?}", location.to_string());

      let astronomy = get_current_astronomy(location.clone()).await.unwrap();
      println!("{:?}", astronomy.to_string());

      Ok(())
    }
    Err(err) => Err(Error::Error {
      message: err.to_string(),
    }),
  }
}

pub async fn handle_current() -> Result<(), Error> {
  println!("Current weather");
  match get_public_ip().await {
    Ok(ip) => {
      let location = get_location_from_ip(&ip).await.unwrap();
      println!("{:?}", location.to_string());

      let weather = get_current_weather(location.clone(), Units::Metric)
        .await
        .unwrap();
      println!("{:?}", weather.to_string());

      Ok(())
    }
    Err(err) => Err(Error::Error {
      message: err.to_string(),
    }),
  }
}

pub async fn handle_forecast(days: i32) -> Result<(), Error> {
  println!("Weather forecast");

  let days_valid = validate_days(days);
  if days_valid.is_err() {
    return days_valid;
  }

  match get_public_ip().await {
    Ok(ip) => {
      let location = get_location_from_ip(&ip).await.unwrap();
      println!("{:?}", location.to_string());

      let weather = get_forecast_weather(location.clone(), Units::Metric, days)
        .await
        .unwrap();
      println!("{:?}", weather.to_string());

      Ok(())
    }
    Err(err) => Err(Error::Error {
      message: err.to_string(),
    }),
  }
}
