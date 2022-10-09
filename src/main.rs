use std::fmt::Display;
#[allow(dead_code)]
pub mod app_data_access;

fn main() {}
#[allow(dead_code)]
/// Shows given messages as error to user and exits the program as failed via
/// returned error code. Is used to react to errors not recoverable
/// in cli without panic in production.
fn exit_with_err_message<T: Display>(message: &T) {
  eprintln!("Error: {}", message);
  std::process::exit(1);
}
