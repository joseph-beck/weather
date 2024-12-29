use weather_core::error::Error;

pub fn validate_days(days: i32) -> Result<(), Error> {
  match days {
    1..=5 => Ok(()),
    _ => Err(Error::InvalidArgument {
      arg: days.to_string(),
      message: "Days should be between 1 and 5.".to_string(),
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_days_valid() {
    assert!(validate_days(1).is_ok());
    assert!(validate_days(3).is_ok());
    assert!(validate_days(5).is_ok());
  }

  #[test]
  fn test_validate_days_invalid() {
    let result = validate_days(0);
    assert!(result.is_err());
    if let Err(Error::InvalidArgument { arg, message }) = result {
      assert_eq!(arg, "0");
      assert_eq!(message, "Days should be between 1 and 5.");
    } else {
      panic!("Expected InvalidArgument error");
    }

    let result = validate_days(6);
    assert!(result.is_err());
    if let Err(Error::InvalidArgument { arg, message }) = result {
      assert_eq!(arg, "6");
      assert_eq!(message, "Days should be between 1 and 5.");
    } else {
      panic!("Expected InvalidArgument error");
    }
  }
}
