use std::fmt::Display;

use clap::Parser;
use stamp_member::args_parser::AppCommand;
use stamp_member::{app_command_impl, app_data_access, data_access};

fn main() {
  normal_app_run();
}

#[cfg(debug_assertions)]
fn initial_with_fake_dev_data() {
  let fake_data = app_data_access::fake_dev_app_data();
  let json = fake_data
    .to_json()
    .expect("Fake data struct can not be serialized to json");
  app_data_access::save_app_data(&json).expect("Saving failed");

  println!(
    "Initial dev data written  to {:?}",
    data_access::paths::get_data_path()
  )
}

fn normal_app_run() {
  let cli_args = AppCommand::parse();

  match cli_args {
    AppCommand::All => match app_command_impl::show_all_items() {
      Ok(table) => println!("{table}"),
      Err(error) => exit_with_err_message(&error),
    },
    AppCommand::Time(args) => match app_command_impl::add_time_stamp_by_title(&args.name) {
      Ok(_) => println!("Time stamp created and created"),
      Err(error) => exit_with_err_message(&error),
    },
    #[cfg(debug_assertions)]
    AppCommand::DevInit => initial_with_fake_dev_data(),
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
