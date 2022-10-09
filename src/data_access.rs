use crate::time_stamp::TimeStamp;
use std::{fs, io};

pub struct TimeStampSource {
  path: String,
}

pub enum DataSourceJsonError {
  FileError(io::Error),
  NotFoundPath(io::Error),
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
      io::ErrorKind::NotFound => DataSourceJsonError::NotFoundPath(error),
      _ => DataSourceJsonError::FileError(error),
    }
  }
}
