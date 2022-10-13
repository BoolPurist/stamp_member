use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{stop_watch::StopWatch, time_stamp::TimeStamp, TimeEntity};

#[derive(Serialize, Deserialize)]
pub struct TimeEntitiesController {
  time_stamps: Vec<TimeStamp>,
  stop_watches: Vec<StopWatch>,
}
const DUPLICATE_ADDED_TIME_ERROR_MSG: &str = "Title already exists on another time stamp";
#[allow(dead_code)]
impl TimeEntitiesController {
  pub fn empty() -> Self {
    TimeEntitiesController {
      time_stamps: Vec::new(),
      stop_watches: Vec::new(),
    }
  }

  pub fn new(time_stamps: Vec<TimeStamp>, stop_watches: Vec<StopWatch>) -> Self {
    TimeEntitiesController {
      time_stamps,
      stop_watches,
    }
  }

  pub fn from_json(json: &str) -> Result<TimeEntitiesController, serde_json::Error> {
    let new = serde_json::from_str(json)?;
    Ok(new)
  }

  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    if cfg!(debug_assertions) {
      let to_return = serde_json::to_string_pretty(self)?;
      Ok(to_return)
    } else {
      let to_return = serde_json::to_string(self)?;
      Ok(to_return)
    }
  }

  pub fn add_new_time_stamp(&mut self, new_title: &str) -> Result<(), &'static str> {
    let duplicate_title_on_stamps = Self::has_duplicate_on(&self.time_stamps, new_title);
    if duplicate_title_on_stamps {
      return Err(DUPLICATE_ADDED_TIME_ERROR_MSG);
    }
    let new_time_stamp = TimeStamp::new(new_title);

    self.time_stamps.push(new_time_stamp);
    Ok(())
  }

  fn has_duplicate_on<T: TimeEntity>(entities: &[T], title: &str) -> bool {
    entities.iter().any(|entity| entity.get_title() == title)
  }
}

impl Display for TimeEntitiesController {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let table_time_stamps = TimeStamp::create_text_table_from_time_stamps(&self.time_stamps);
    writeln!(f, "Time stamps: \n{table_time_stamps}")
  }
}
#[cfg(test)]
mod test {
  use super::*;
  use crate::time_entities::time_stamp::TimeStamp;

  fn create_fake_timestamps() -> Vec<TimeStamp> {
    vec![
      TimeStamp::new("1"),
      TimeStamp::new("2"),
      TimeStamp::new("3"),
    ]
  }

  #[test]
  fn should_add_new_time_stamp_unique_title() {
    let time_stamps = create_fake_timestamps();
    let mut time_container = TimeEntitiesController::new(time_stamps, Vec::new());

    let result = time_container.add_new_time_stamp(&"Unique");

    match result {
      Ok(_) => (),
      Err(_) => panic!("Could not add new time stamp as with new unique title."),
    }
  }

  #[test]
  fn should_get_error_with_adding_duplicate() {
    let time_stamps = create_fake_timestamps();
    let mut time_container = TimeEntitiesController::new(time_stamps, Vec::new());

    let result = time_container.add_new_time_stamp(&"1");

    match result {
      Ok(_) => panic!("Duplicate title added did not raise an error"),
      Err(msg) => assert_eq!(DUPLICATE_ADDED_TIME_ERROR_MSG, msg),
    }
  }
}
