use std::{
  io,
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
