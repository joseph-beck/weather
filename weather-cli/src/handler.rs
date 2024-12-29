use weather_core::{
  astronomy::get_current_astronomy,
  error::Error,
  location::get_location_from_ip,
  weather::{get_current_weather, get_forecast_weather, Units},
};

pub async fn handle_astronomy() -> Result<(), Error> {
  println!("Current astronomy");
  match weather_core::ip::get_public_ip().await {
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
  match weather_core::ip::get_public_ip().await {
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
  match weather_core::ip::get_public_ip().await {
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
