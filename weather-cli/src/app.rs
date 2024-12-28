use weather_core::weather::get_current_weather;

pub async fn run() -> Result<(), weather_core::error::Error> {
  dotenv::dotenv().ok();

  match weather_core::ip::get_public_ip().await {
    Ok(ip) => {
      let location = weather_core::location::Location::from_ip(&ip)
        .await
        .unwrap();
      println!("{:?}", location);

      let weather = get_current_weather(location, weather_core::weather::Units::Metric)
        .await
        .unwrap();
      println!("{:?}", weather);

      Ok(())
    }
    Err(err) => Err(weather_core::error::Error::Error {
      message: err.to_string(),
    }),
  }
}
