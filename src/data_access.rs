use crate::time_stamp::TimeStamp;
use std::{
  fs, io,
  path::{Path, PathBuf},
};

pub struct TimeStampSource {
  path: String,
}
const DEV_PATH_RESOURCES: &str = "dev_resources";
const NAME_DATA_FILE: &str = "data.json";
pub fn get_data_path() -> io::Result<PathBuf> {
  if cfg!(debug_assertions) {
    return Ok(get_dev_path_data());
  } else {
    let executable_path = std::env::current_exe()?;
    return Ok(executable_path);
  }
}

fn get_dev_path_data() -> PathBuf {
  let project_path: &str = env!("CARGO_MANIFEST_DIR");
  Path::new(project_path)
    .join(DEV_PATH_RESOURCES)
    .join(NAME_DATA_FILE)
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
