use crate::time_stamp::TimeStamp;
use std::{
  fmt::Display,
  fs, io,
  path::{Path, PathBuf},
};

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
#[derive(Debug)]
pub enum DataSourceJsonError {
  FileError(io::Error),
  NotFoundAtPath(io::Error),
  JsonSerdeError(serde_json::Error),
}
impl Display for DataSourceJsonError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DataSourceJsonError::FileError(err) => writeln!(f, "Error with file handling.\n{err}"),
      DataSourceJsonError::NotFoundAtPath(err) => writeln!(f, "Error with file handling.\n{err}"),
      DataSourceJsonError::JsonSerdeError(err) => writeln!(f, "Can not read invalid json.\n{err}"),
    }
  }
}
pub type AppDataJsonResult<T> = Result<T, DataSourceJsonError>;
pub fn get_all_data(path: &PathBuf) -> AppDataJsonResult<Vec<TimeStamp>> {
  let content = fs::read_to_string(path)?;
  let data: Vec<TimeStamp> = serde_json::from_str(&content)?;
  Ok(data)
}

pub fn save_data(path: &PathBuf, data: &[TimeStamp]) -> AppDataJsonResult<()> {
  let data_as_json = serde_json::to_string(data)?;
  fs::write(path, data_as_json)?;
  Ok(())
}

impl From<serde_json::Error> for DataSourceJsonError {
  fn from(error: serde_json::Error) -> Self {
    DataSourceJsonError::JsonSerdeError(error)
  }
}

impl From<io::Error> for DataSourceJsonError {
  fn from(error: io::Error) -> Self {
    match error.kind() {
      io::ErrorKind::NotFound => DataSourceJsonError::NotFoundAtPath(error),
      _ => DataSourceJsonError::FileError(error),
    }
  }
}
