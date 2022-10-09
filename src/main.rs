use std::fmt::Display;

use clap::Parser;
use stamp_member::{
  args_parser::AppCommand, data_access::AppDataJsonResult, time_stamp::TimeStamp,
};
#[allow(dead_code)]
pub mod app_data_access;

fn main() {
  let cli_args = AppCommand::parse();
  match cli_args {
    AppCommand::All => {
      if let Err(error) = show_all_items() {
        exit_with_err_message(&error)
      }
    }
    _ => exit_with_err_message(&"Not implemented yet"),
  }
}
#[allow(dead_code)]
/// Shows given messages as error to user and exits the program as failed via
/// returned error code. Is used to react to errors not recoverable
/// in cli without panic in production.
fn exit_with_err_message<T: Display>(message: &T) {
  eprintln!("Error: {}", message);
  std::process::exit(1);
}

fn show_all_items() -> AppDataJsonResult<()> {
  let data = app_data_access::read_app_data()?;
  let table = TimeStamp::create_text_table_from_time_stamps(&data);
  println!("{table}");
  Ok(())
}
