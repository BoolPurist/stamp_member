#[cfg(test)]
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
