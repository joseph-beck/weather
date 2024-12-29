use weather_core::{
  altert::get_alerts,
  astronomy::get_current_astronomy,
  error::Error,
  ip::get_public_ip,
  location::get_location_from_ip,
  weather::{get_current_weather, get_forecast_weather, Units},
};

pub async fn handle_alerts(days: i32) -> Result<(), Error> {
  println!("Weather alerts");
  match get_public_ip().await {
    Ok(ip) => {
      let location = get_location_from_ip(&ip).await.unwrap();
      println!("{:?}", location);

      let alerts = get_alerts(location.clone(), days).await.unwrap();
      println!("{:?}", alerts);

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
      println!("{:?}", location);

      let astronomy = get_current_astronomy(location.clone()).await.unwrap();
      println!("{:?}", astronomy);

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
      println!("{:?}", location);

      let weather = get_current_weather(location.clone(), Units::Metric)
        .await
        .unwrap();
      println!("{:?}", weather);

      Ok(())
    }
    Err(err) => Err(Error::Error {
      message: err.to_string(),
    }),
  }
}

pub async fn handle_forecast(days: i32) -> Result<(), Error> {
  println!("Weather forecast");
  match get_public_ip().await {
    Ok(ip) => {
      let location = get_location_from_ip(&ip).await.unwrap();
      println!("{:?}", location);

      let weather = get_forecast_weather(location.clone(), Units::Metric, days)
        .await
        .unwrap();
      println!("{:?}", weather);

      Ok(())
    }
    Err(err) => Err(Error::Error {
      message: err.to_string(),
    }),
  }
}
