use clap::Command;

pub fn cli() -> Command<'static> {
  Command::new("weather")
    .about("Weather!")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(Command::new("astronomy").about("Get the current astronomy."))
    .subcommand(Command::new("current").about("Get the current weather."))
}
