use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{stop_watch::StopWatch, time_stamp::TimeStamp};

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

  pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
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
}

impl Display for TimeEntitiesController {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let table_time_stamps = TimeStamp::create_text_table_from_time_stamps(&self.time_stamps);
    writeln!(f, "Time stamps: \n{table_time_stamps}")
  }
}
