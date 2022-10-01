pub mod args_parser_tests;
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
  let args_len = args.len();
  if args_len == 0 {
    return Ok(Command::ShowAll);
  } else if args_len > 3 {
    return Err(ErrorCommand::TooManyArgs(args_len as u32));
  }

  let command_as_text: &str = args.first().unwrap();
  match command_as_text {
    SHOW_COMMAND => {
      if args_len > 1 {
        let title = args[1].clone();
        return Ok(Command::ShowItem(title));
      } else {
        return Ok(Command::ShowAll);
      }
    }
    _ => Err(ErrorCommand::UnknownCommand(String::from(command_as_text))),
  }
}
