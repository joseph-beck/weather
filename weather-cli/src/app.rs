use weather_core::astronomy::get_current_astronomy;
use weather_core::location::get_location_from_ip;
use weather_core::weather::{get_current_weather, Units};

pub async fn run() -> Result<(), weather_core::error::Error> {
  dotenv::dotenv().ok();

  match weather_core::ip::get_public_ip().await {
    Ok(ip) => {
      let location = get_location_from_ip(&ip).await.unwrap();
      println!("{:?}", location);

      let weather = get_current_weather(location.clone(), Units::Metric)
        .await
        .unwrap();
      println!("{:?}", weather);

      let astronomy = get_current_astronomy(location.clone()).await.unwrap();
      println!("{:?}", astronomy);

      Ok(())
    }
    Err(err) => Err(weather_core::error::Error::Error {
      message: err.to_string(),
    }),
  }
}
