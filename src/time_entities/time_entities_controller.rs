use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{stop_watch::StopWatch, time_stamp::TimeStamp, TimeEntity};

#[derive(Serialize, Deserialize)]
pub struct TimeEntitiesController {
  time_stamps: Vec<TimeStamp>,
  stop_watches: Vec<StopWatch>,
}
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
      return Err("Title already exists on another time stamp");
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
