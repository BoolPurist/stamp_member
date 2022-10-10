use std::{fs, io, path::PathBuf};

pub mod paths;
pub fn get_all_data(path: &PathBuf) -> io::Result<String> {
  let content = fs::read_to_string(path)?;
  Ok(content)
}

pub fn save_data(path: &PathBuf, data_json: &str) -> io::Result<()> {
  fs::write(path, data_json)?;
  Ok(())
}
