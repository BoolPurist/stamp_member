pub enum Command {
  CreateTimeStamp(String),
  CreateStopWatch(String, String),
  ShowItem(String),
  DeleteItem(String),
  StopItem(String),
  ResumeItem(String),
  ShowAll,
}

pub enum ErrorCommand {
  TooManyArgs(u32),
  UnknownCommand(String),
  MissingArg(String, u32),
}

const SHOW_COMMAND: &str = "show";

pub fn figure_command_from_args(args: Vec<String>) -> Result<Command, ErrorCommand> {
  if args.len() == 0 {
    return Ok(Command::ShowAll);
  }

  let command_as_text: &str = args.first().unwrap();
  match command_as_text {
    SHOW_COMMAND => {
      if args.len() > 1 {
        let title = args[1].clone();
        return Ok(Command::ShowItem(title));
      } else {
        return Ok(Command::ShowAll);
      }
    }
    _ => Err(ErrorCommand::UnknownCommand(String::from(command_as_text))),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn shows_all_without_args() {
    let no_args: Vec<String> = Vec::new();

    if let Ok(Command::ShowAll) = figure_command_from_args(no_args) {
      return;
    }
    panic!("show all without arguments was not selected as default");
  }

  #[test]
  fn figure_show_one_item() {
    let expected_title = "Some title";
    let show_with_title: Vec<String> = vec![SHOW_COMMAND.to_string(), expected_title.to_string()];

    if let Ok(Command::ShowItem(title)) = figure_command_from_args(show_with_title) {
      assert_eq!(title, expected_title);
      return;
    }
    panic!(
      "Should take command for showing item with title ({})",
      &expected_title
    )
  }

  #[test]
  fn figure_show_all_item_without_title() -> Result<(), String> {
    let command_without_any_title = vec![SHOW_COMMAND.to_string()];

    if let Ok(Command::ShowAll) = figure_command_from_args(command_without_any_title) {
      return Ok(());
    }

    return Err(format!(
      "Should all show all items without any title for command {}",
      SHOW_COMMAND
    ));
  }
}
