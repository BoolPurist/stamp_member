use std::fmt::Display;

use chrono::{DateTime, Utc};
pub struct StampOperationError<T> {
  error_message: &'static str,
  pub error_kind: T,
}

impl<T> StampOperationError<T> {
  pub fn get_error_msg(&self) -> &str {
    &self.error_message
  }

  pub fn new(error_message: &'static str, error_kind: T) -> Self {
    StampOperationError {
      error_message,
      error_kind,
    }
  }
}

pub enum StopError<'a> {
  IsFinishedAlready(&'a DateTime<Utc>),
  IsStoppedAlready(&'a DateTime<Utc>),
}

impl<T> Display for StampOperationError<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.get_error_msg())
  }
}
