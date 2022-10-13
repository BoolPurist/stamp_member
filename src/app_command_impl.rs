pub mod app_command_errors;

use crate::{
  app_data_access,
  time_entities::time_entities_controller::{DuplicateTitleError, TimeEntitiesController},
};

use self::app_command_errors::{AppDataIoOrJsonError, AppDataOperationError};

pub fn show_all_items() -> Result<String, AppDataIoOrJsonError> {
  let data = app_data_access::read_app_data()?;
  let entities = TimeEntitiesController::from_json(&data)?;
  Ok(format!("{entities}"))
}

pub fn add_time_stamp_by_title(
  new_title: &str,
) -> Result<(), AppDataOperationError<DuplicateTitleError>> {
  let data = app_data_access::read_app_data()?;
  let mut entities = TimeEntitiesController::from_json(&data)?;
  let result = entities.add_new_time_stamp(new_title);

  match result {
    Ok(_) => {
      let json = entities.to_json()?;
      app_data_access::save_app_data(&json)?;
      Ok(())
    }
    Err(_) => Err(AppDataOperationError::OperationErrorOnEntity(
      DuplicateTitleError,
    )),
  }
}

// pub fn pause_by_title() -> Result<(), AppDataOperationError> {
// let data = app_data_access::read_app_data()?;
// let mut entities = TimeEntitiesController::from_json(&data)?;
// }
