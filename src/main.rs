use stamp_member::args_parser::{figure_command_from_args, Command};
fn main() {
  let provided_args: Vec<String> = std::env::args().skip(1).collect();

  match figure_command_from_args(provided_args) {
    Ok(success) => match success {
      Command::ShowAll => println!("ShowAll"),
      _ => println!("Success"),
    },
    Err(_error) => println!("error"),
  }
}
