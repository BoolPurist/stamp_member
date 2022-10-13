use std::{fmt::Display, io};

use crate::{app_data_access, time_entities::time_entities_controller::TimeEntitiesController};

pub enum AppDataOperationError {
  IoError(io::Error),
  JsonError(serde_json::Error),
  DuplicateTitle(&'static str),
}
pub fn show_all_items() -> Result<String, AppDataOperationError> {
  let data = app_data_access::read_app_data()?;
  let entities = TimeEntitiesController::from_json(&data)?;
  Ok(format!("{entities}"))
}

pub fn add_time_stamp_by_title(new_title: &str) -> Result<(), AppDataOperationError> {
  let data = app_data_access::read_app_data()?;
  let mut entities = TimeEntitiesController::from_json(&data)?;
  let result = entities.add_new_time_stamp(new_title);

  match result {
    Ok(_) => {
      let json = entities.to_json()?;
      app_data_access::save_app_data(&json)?;
      Ok(())
    }
    Err(message) => Err(AppDataOperationError::DuplicateTitle(message)),
  }
}

// pub fn pause_by_title() -> Result<(), AppDataOperationError> {
// let data = app_data_access::read_app_data()?;
// let mut entities = TimeEntitiesController::from_json(&data)?;
// }

impl From<io::Error> for AppDataOperationError {
  fn from(error: io::Error) -> Self {
    AppDataOperationError::IoError(error)
  }
}

impl From<serde_json::Error> for AppDataOperationError {
  fn from(error: serde_json::Error) -> Self {
    AppDataOperationError::JsonError(error)
  }
}

impl Display for AppDataOperationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AppDataOperationError::IoError(error) => writeln!(f, "{error}"),
      AppDataOperationError::JsonError(error) => writeln!(f, "{error}"),
      AppDataOperationError::DuplicateTitle(error) => writeln!(f, "{error}"),
    }
  }
}
