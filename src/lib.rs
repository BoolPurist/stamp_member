pub mod args_parser;
pub mod format_utils;
pub mod time_stamp;

use std::{fs, io};
use time_stamp::TimeStamp;
pub struct TimeStampSource {
  path: String,
}

pub enum DataSourceJsonError {
  FileError(io::ErrorKind),
  NotFoundPath,
  JsonSerdeError(serde_json::Error),
}

impl TimeStampSource {
  pub fn new(path: &str) -> TimeStampSource {
    TimeStampSource {
      path: path.to_string(),
    }
  }

  pub fn get_all(&self) -> Result<Vec<TimeStamp>, DataSourceJsonError> {
    let content = fs::read_to_string(&self.path)?;
    let data: Vec<TimeStamp> = serde_json::from_str(&content)?;
    Ok(data)
  }
}

impl From<serde_json::Error> for DataSourceJsonError {
  fn from(error: serde_json::Error) -> Self {
    DataSourceJsonError::JsonSerdeError(error)
  }
}

impl From<io::Error> for DataSourceJsonError {
  fn from(error: io::Error) -> Self {
    match error.kind() {
      io::ErrorKind::NotFound => DataSourceJsonError::NotFoundPath,
      _ => DataSourceJsonError::FileError(error.kind()),
    }
  }
}
