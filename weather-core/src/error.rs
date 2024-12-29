#[derive(Debug, PartialEq)]
/// Various Error types that can occur when using the weather-core crate.
pub enum Error {
  Error { message: String },
  Fetch { message: String },
  Response { message: String },
  BadIp { ip: String, message: String },
  InvalidArgument { arg: String, message: String },
  UnknownLocation { location: String },
  NoLocation,
}
