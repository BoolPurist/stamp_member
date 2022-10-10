use crate::{
  app_data_access, data_access::AppDataJsonResult,
  time_entities::time_entities_controller::TimeEntitiesController,
};

pub fn show_all_items() -> AppDataJsonResult<()> {
  let data = app_data_access::read_app_data()?;
  let entities = TimeEntitiesController::from_json(&data)?;
  println!("{entities}");
  Ok(())
}
