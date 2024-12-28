pub mod astronomy;
pub mod error;
pub mod ip;
pub mod location;
pub mod weather;

pub trait FromResponse<T> {
  fn new_from_response(response: T) -> Self;
}

pub trait FromResponseWithOption<T, O> {
  fn new_from_response_with_options(response: T, option: O) -> Self;
}
