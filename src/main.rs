use std::fmt::Display;

use clap::Parser;
use stamp_member::app_command_impl;
use stamp_member::args_parser::AppCommand;

fn main() {
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
