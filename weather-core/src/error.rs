#[derive(Debug)]
pub enum Error {
  Error { message: String },
  Fetch { message: String },
  Response { message: String },
  BadIp { ip: String, message: String },
  UnknownLocation { location: String },
}
