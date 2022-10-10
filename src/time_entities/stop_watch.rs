use serde::{Deserialize, Serialize};

use super::time_stamp::TimeStamp;

#[derive(Serialize, Deserialize)]
pub struct StopWatch {
  time_stamp: TimeStamp,
  time_left: Option<usize>,
}
