use weather_core::error::Error;

use crate::command::cli;
use crate::handler::{handle_astronomy, handle_current};

pub async fn run() -> Result<(), Error> {
  dotenv::dotenv().ok();

  let matches = cli().get_matches();

  match matches.subcommand() {
    Some(("astronomy", _)) => match handle_astronomy().await {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    },
    Some(("current", _)) => match handle_current().await {
      Ok(_) => Ok(()),
      Err(err) => Err(err),
    },
    _ => unreachable!(),
  }
}
