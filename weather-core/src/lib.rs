use error::Error;

pub mod alert;
pub mod astronomy;
pub mod error;
pub mod ip;
pub mod location;
pub mod weather;

/// Query tratif ro generating a query result for a generti type `T`
/// Uses self to generate the query.
pub trait Query<T> {
  /// Generate the query result for a generic type `T`.
  /// Can return an error if unable to create the query.
  /// Otherwise, the query result is returned.
  fn query(&self) -> Result<T, Error>;
}
