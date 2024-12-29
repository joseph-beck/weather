use clap::{value_parser, Arg, Command};

pub fn cli() -> Command<'static> {
  // Verbose option. Defaults to false.
  let verbose_option = Arg::new("verbose")
    .short('v')
    .long("verbose")
    .value_parser(value_parser!(bool))
    .default_value("false")
    .required(false)
    .help("Output information more verbosely.");

  // City option. Defaults to None.
  let city_option = Arg::new("city")
    .short('c')
    .long("city")
    .value_parser(value_parser!(String))
    .required(false)
    .help("City to get the weather for.");

  // Post code option. Defaults to None.
  let post_code_option = Arg::new("post_code")
    .short('p')
    .long("post_code")
    .value_parser(value_parser!(String))
    .required(false)
    .help("Post code to get the weather for.");

  // Days argument. Defaults to 1.
  let days_arg = Arg::new("days")
    .value_parser(value_parser!(i32))
    .default_value("1")
    .validator(|v| {
      let value = v.parse::<i32>().unwrap();
      if value < 1 || value > 5 {
        Err("Days must be between 1 and 5.".to_string())
      } else {
        Ok(())
      }
    })
    .required(false)
    .help("How many days ahead to forecast to? Minimum value of 1 and maximum value of 5.");

  Command::new("weather")
    .about("Weather!")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("alert")
        .about("Get the current alerts.")
        .arg(days_arg.clone())
        .arg(verbose_option.clone())
        .arg(city_option.clone())
        .arg(post_code_option.clone()),
    )
    .subcommand(
      Command::new("astronomy")
        .about("Get the current astronomy.")
        .arg(verbose_option.clone())
        .arg(city_option.clone())
        .arg(post_code_option.clone()),
    )
    .subcommand(
      Command::new("current")
        .about("Get the current weather.")
        .arg(verbose_option.clone())
        .arg(city_option.clone())
        .arg(post_code_option.clone()),
    )
    .subcommand(
      Command::new("forecast")
        .about("Get the forecasted weather.")
        .arg(days_arg.clone())
        .arg(verbose_option.clone())
        .arg(city_option.clone())
        .arg(post_code_option.clone()),
    )
}
