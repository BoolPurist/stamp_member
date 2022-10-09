use std::fmt::Display;
#[allow(dead_code)]
pub mod app_data_access;

fn main() {}
#[allow(dead_code)]
fn exit_with_err_message<T: Display>(message: &T) {
  eprintln!("Error: {}", message);
  std::process::exit(1);
}
