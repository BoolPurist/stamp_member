use std::{fmt::Display, io};
#[derive(Debug)]
pub enum AppDataIoOrJsonError {
  IoError(io::Error),
  JsonError(serde_json::Error),
}

#[derive(Debug)]
pub enum AppDataOperationError<T>
where
  T: Display,
{
  IoOrJsonError(AppDataIoOrJsonError),
  OperationErrorOnEntity(T),
}

impl From<io::Error> for AppDataIoOrJsonError {
  fn from(error: io::Error) -> Self {
    AppDataIoOrJsonError::IoError(error)
  }
}

impl From<serde_json::Error> for AppDataIoOrJsonError {
  fn from(error: serde_json::Error) -> Self {
    AppDataIoOrJsonError::JsonError(error)
  }
}

impl<T> From<io::Error> for AppDataOperationError<T>
where
  T: Display,
{
  fn from(error: io::Error) -> Self {
    AppDataOperationError::IoOrJsonError(AppDataIoOrJsonError::IoError(error))
  }
}

impl<T> From<serde_json::Error> for AppDataOperationError<T>
where
  T: Display,
{
  fn from(error: serde_json::Error) -> Self {
    AppDataOperationError::IoOrJsonError(AppDataIoOrJsonError::JsonError(error))
  }
}

impl Display for AppDataIoOrJsonError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AppDataIoOrJsonError::IoError(error) => write!(f, "{}", error),
      AppDataIoOrJsonError::JsonError(error) => write!(f, "{}", error),
    }
  }
}

impl<T> Display for AppDataOperationError<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AppDataOperationError::IoOrJsonError(error) => writeln!(f, "{error}"),
      AppDataOperationError::OperationErrorOnEntity(error) => writeln!(f, "{error}"),
    }
  }
}
