use std::fmt::Display;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
pub mod time_stamp_errors;
#[cfg(test)]
mod time_stamp_tests;

use crate::format_utils;

use self::time_stamp_errors::{StampOperationError, StopError};

use super::TimeEntity;
const ERROR_MSG_ALREADY_PAUSED: &str = "Is already stopped";
const ERROR_MSG_ALREADY_FINISHED: &str = "Is already finished";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TimeStamp {
  title: String,
  started: DateTime<Utc>,
  ended: Option<DateTime<Utc>>,
  is_paused: bool,
  last_paused: Option<DateTime<Utc>>,
  #[cfg(test)]
  #[serde(skip)]
  /// Used in tests for functions which work with current moment in time. Example Pause, Finish etc
  /// This allow manipulation of the current moment in time manually
  current_fake_now_moment: DateTime<Utc>,
}

impl TimeStamp {
  /// Used to show to user that a moment in time is not known yet.
  /// # Example
  /// If a time stamp is not finished yet then ended time is shown as NOT_AVAILABLE to the user.
  const NOT_AVAILABLE: &'static str = "N/A";
  pub fn new(title: &str) -> TimeStamp {
    TimeStamp::with_started(title, Utc::now())
  }

  pub fn with_started(title: &str, started: DateTime<Utc>) -> TimeStamp {
    TimeStamp {
      #[cfg(test)]
      current_fake_now_moment: started.clone(),
      title: title.trim().to_string(),
      started,
      ended: None,
      is_paused: false,
      last_paused: None,
    }
  }

  fn get_text_headers() -> Vec<String> {
    vec![
      "Title",
      "Started at",
      "Ended at",
      "Is paused",
      "Last time paused",
    ]
    .iter()
    .map(|to_str| to_str.to_string())
    .collect()
  }

  /// Paused a time stamp. This stops the counting of the passed time on this stamp. Instead it will
  /// count the paused time until resumed, ending the pause.
  /// # Errors
  /// If stamp is already paused or finished.
  pub fn pause(&mut self) -> Result<&DateTime<Utc>, StampOperationError<StopError>> {
    if self.is_paused {
      return Err(StampOperationError::new(
        ERROR_MSG_ALREADY_PAUSED,
        StopError::IsStoppedAlready(self.last_paused.as_ref().unwrap()),
      ));
    }

    match self.ended {
      Some(ref time_ended) => Err(StampOperationError::new(
        ERROR_MSG_ALREADY_FINISHED,
        StopError::IsFinishedAlready(time_ended),
      )),
      None => {
        self.is_paused = true;
        let new_last_paused = self.get_now().clone();
        self.last_paused = Some(new_last_paused);
        Ok(self.last_paused.as_ref().unwrap())
      }
    }
  }

  /// Finishes a time stamp. Time stamp can not be paused or resumed after this invocation.
  /// Finished time stamps will save the moment in time at which they are finished.
  /// This time moment will not change.
  /// This moment is always returned even if an error is returned.
  /// # Errors
  /// Calling this method a second time. Because a finished time stamp can not be finished again.
  pub fn finish(&mut self) -> Result<&DateTime<Utc>, &DateTime<Utc>> {
    self.is_paused = false;
    match self.ended {
      Some(ref ended_time) => Err(ended_time),
      None => {
        self.ended = Some(self.get_now().clone());
        Ok(self.ended.as_ref().unwrap())
      }
    }
  }

  pub fn create_text_table_from_time_stamps(data: &Vec<TimeStamp>) -> String {
    let mut text_data = TimeStamp::many_to_text(data);
    let headers = TimeStamp::get_text_headers();

    text_data.insert(0, headers);
    format_utils::format_to_text_table(&text_data, 2)
  }

  /// Creates a list of text columns from time stamps. Useful for preparing time stamps for
  /// other functions to print tables.
  fn many_to_text(to_convert: &[TimeStamp]) -> Vec<Vec<String>> {
    let mut to_return: Vec<Vec<String>> = Vec::new();
    for time_stamp in to_convert {
      let columns = time_stamp.to_str_vec();
      to_return.push(columns);
    }
    to_return
  }

  /// Outputs a vector with text row with columns for:
  /// - Title: Label of timestamp/stopwatch.
  /// - Started: Time at which it was started.
  /// - Ended: Time at wich the stopwatch has ended or the timestamp was finished.
  /// N/A if it was not ended yet
  /// - Is paused: yes for stopped. no if not stopped.
  /// - Time left: Seconds left until stopwatch is done. N/A if not stopwatch
  fn to_str_vec(&self) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    output.push(self.title.clone());
    output.push(TimeStamp::time_to_str(self.started));

    TimeStamp::push_text_date_time(&mut output, self.ended);

    output.push(if self.is_paused {
      "yes".to_string()
    } else {
      "no".to_string()
    });

    TimeStamp::push_text_date_time(&mut output, self.last_paused);

    output
  }

  fn push_text_date_time<T: Display>(to_push_on: &mut Vec<String>, date_time: Option<T>) {
    if let Some(left_time) = date_time {
      to_push_on.push(left_time.to_string())
    } else {
      to_push_on.push(TimeStamp::NOT_AVAILABLE.to_string())
    }
  }

  /// Returns an easy to read text presentation of date time.
  /// The date is shown as date, (month, day, year) and
  /// then time (hour minutes seconds) from left to right.
  fn time_to_str(date: DateTime<Utc>) -> String {
    let seconds = format_utils::with_at_least_2_digits(date.second() as u64);
    let minutes = format_utils::with_at_least_2_digits(date.minute() as u64);
    let hours = format_utils::with_at_least_2_digits(date.hour() as u64);
    let days = format_utils::with_at_least_2_digits(date.day() as u64);
    let months = format_utils::with_at_least_2_digits(date.month() as u64);

    format!(
      "On {}.{}.{} at {}:{}:{}",
      months,
      days,
      date.year(),
      hours,
      minutes,
      seconds
    )
  }

  #[cfg(not(test))]
  fn get_now(&self) -> DateTime<Utc> {
    Utc::now()
  }

  #[cfg(test)]
  fn get_now(&self) -> &DateTime<Utc> {
    &self.current_fake_now_moment
  }
}

impl TimeEntity for TimeStamp {
  fn get_title(&self) -> &str {
    &self.title
  }
}
