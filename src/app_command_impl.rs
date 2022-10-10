use crate::{app_data_access, data_access::AppDataJsonResult, time_stamp::TimeStamp};

pub fn show_all_items() -> AppDataJsonResult<()> {
  let data = app_data_access::read_app_data()?;
  let table = TimeStamp::create_text_table_from_time_stamps(&data);
  println!("{table}");
  Ok(())
}
