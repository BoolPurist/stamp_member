use clap::{Args, Parser};
#[derive(Args, Debug)]
pub struct Title {
  /// Name of time stamp or stop watch
  name: String,
}
#[derive(Args, Debug)]
pub struct StopWatch {
  /// Name of stop watch
  name: String,
  /// Count down until watch is done.
  left_time: String,
}
#[derive(Parser, Debug)]
#[command(author = "NiceGraphic", version = "1.0.0", about="Tool to manage timestamps", long_about = None)]
pub enum AppCommand {
  /// Creates a time stamp which starts from current time.
  Time(Title),
  /// Creates a stop watch which starts from current time and given count down.
  Watch(StopWatch),
  /// Show time stamp or stop watch of a given title.
  Show(Title),
  /// Deletes time stamp or stop watch with the given title
  Delete(Title),
  /// Stops time stamp and stop watch. Stops counting time.
  Stop(Title),
  /// Continues times stamp or stop watch if stopped before. Continues counting
  /// again.
  Resume(Title),
  /// Lists all created time stamps and stop watches.
  All,
}
