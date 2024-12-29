use weather_core::error::Error;

use crate::command::cli;
use crate::handler::{handle_alerts, handle_astronomy, handle_current, handle_forecast};

pub async fn run() -> Result<(), Error> {
  dotenv::dotenv().ok();

  let matches = cli().get_matches();

  match matches.subcommand() {
    Some(("alert", args)) => match handle_alerts(*args.get_one::<i32>("days").unwrap()).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    },
    Some(("astronomy", _)) => match handle_astronomy().await {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    },
    Some(("current", _)) => match handle_current().await {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    },
    Some(("forecast", args)) => {
      match handle_forecast(*args.get_one::<i32>("days").unwrap()).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
      }
    }
    _ => unreachable!(),
  }
}
