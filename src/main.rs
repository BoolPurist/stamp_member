use std::fmt::Display;

use clap::Parser;
use stamp_member::args_parser::AppCommand;
use stamp_member::{app_command_impl, app_data_access};

fn main() {
  normal_app_run();
}

#[allow(dead_code)]
fn initial_with_fake_dev_data() {
  let fake_data = app_data_access::fake_dev_app_data();
  let json = fake_data.to_json().unwrap();
  app_data_access::save_app_data(&json).expect("Saving failed");
}

#[allow(dead_code)]
fn normal_app_run() {
  let cli_args = AppCommand::parse();

  match cli_args {
    AppCommand::All => {
      if let Err(error) = app_command_impl::show_all_items() {
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
