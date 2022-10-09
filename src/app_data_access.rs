use chrono::prelude::*;
use stamp_member::data_access::{self, AppDataJsonResult};
use stamp_member::time_stamp::TimeStamp;

fn read_app_data() -> AppDataJsonResult<Vec<TimeStamp>> {
  let path = data_access::get_data_path()?;
  let data = data_access::get_all_data(&path)?;
  Ok(data)
}

fn save_app_data(data: &[TimeStamp]) -> AppDataJsonResult<()> {
  let path = data_access::get_data_path()?;
  data_access::save_data(&path, &data)?;
  Ok(())
}
#[allow(dead_code)]
fn dev_data_vec() -> Vec<TimeStamp> {
  vec![
    TimeStamp::with_started("Hello from 1. line", Utc.ymd(2012, 8, 2).and_hms(23, 34, 2)),
    TimeStamp::with_started(
      "World form 2. line",
      Utc.ymd(2023, 1, 30).and_hms(12, 11, 3),
    ),
    TimeStamp::with_started("Bye form 3. line", Utc.ymd(2002, 4, 12).and_hms(12, 2, 22)),
  ]
}
