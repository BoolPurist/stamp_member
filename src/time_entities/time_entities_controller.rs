use serde::{Deserialize, Serialize};

use super::{stop_watch::StopWatch, time_stamp::TimeStamp};

#[derive(Serialize, Deserialize)]
struct TimeEntitiesController {
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

  pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
    let new = serde_json::from_str(json)?;
    Ok(new)
  }
}
