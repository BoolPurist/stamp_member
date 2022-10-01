use chrono::prelude::*;

pub mod args_parser;

fn create_local_from_utc(utc: &DateTime<Utc>) -> DateTime<Local> {
  utc.with_timezone(&Local)
}
