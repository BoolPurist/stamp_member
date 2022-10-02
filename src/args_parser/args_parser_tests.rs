use super::*;

#[test]
fn detect_too_many_args() {
  const EXPECTED_NUMBER: u32 = 4;
  let args_as_four = vec![
    "show".to_string(),
    "title".to_string(),
    "23:32".to_string(),
    "zero".to_string(),
  ];
  if let Err(ErrorCommand::TooManyArgs(number)) = figure_command_from_args(args_as_four) {
    assert_eq!(EXPECTED_NUMBER, number);
    return;
  }
  panic!(
    "Should have detect too many arguments with number {}",
    EXPECTED_NUMBER
  );
}

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
  make_test_for_command_with_title(SHOW_COMMAND, |result| {
    if let Ok(Command::ShowItem(title)) = result {
      Some(title)
    } else {
      None
    }
  });
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

#[test]
fn detect_unknown_command() {
  const UNKNOWN_COMMAND_NAME: &str = "Unknown";
  let args = vec![UNKNOWN_COMMAND_NAME.to_string()];
  if let Err(ErrorCommand::UnknownCommand(unknown_name)) = figure_command_from_args(args) {
    assert_eq!(UNKNOWN_COMMAND_NAME, unknown_name);
    return;
  }
  panic!(
    "{} was not detected as unknown command.",
    UNKNOWN_COMMAND_NAME
  )
}

const EXPECTED_TITLE: &str = "Expected title.";
fn make_test_for_command_with_title<F>(name: &str, match_for_command: F)
where
  F: Fn(Result<Command, ErrorCommand>) -> Option<String>,
{
  let result = make_setup_for_command_with_title(name);
  let option = match_for_command(result);
  match option {
    Some(title) => {
      assert_for_expected_title(&title);
      return;
    }
    None => panic_for_mismatched_command_or_title(name),
  }
}
fn assert_for_expected_title(actual_title: &str) {
  assert_eq!(actual_title, EXPECTED_TITLE);
}
fn panic_for_mismatched_command_or_title(name: &str) {
  panic!("Did not extract command with name {name} and a title")
}
fn make_setup_for_command_with_title(command: &str) -> Result<Command, ErrorCommand> {
  let show_with_title: Vec<String> = vec![command.to_string(), EXPECTED_TITLE.to_string()];
  figure_command_from_args(show_with_title)
}
