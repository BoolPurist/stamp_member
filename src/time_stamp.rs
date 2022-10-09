use std::fmt::Display;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::format_utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeStamp {
  title: String,
  started: DateTime<Utc>,
  ended: Option<DateTime<Utc>>,
  finished: Option<DateTime<Utc>>,
  is_paused: bool,
  last_paused: Option<DateTime<Utc>>,
  time_left: Option<usize>,
}

impl TimeStamp {
  const NOT_AVAILABLE: &'static str = "N/A";
  pub fn new(title: &str) -> TimeStamp {
    TimeStamp {
      title: title.to_string(),
      started: Utc::now(),
      ended: None,
      finished: None,
      is_paused: false,
      last_paused: None,
      time_left: None,
    }
  }

  pub fn with_started(title: &str, started: DateTime<Utc>) -> TimeStamp {
    TimeStamp {
      title: title.to_string(),
      started,
      ended: None,
      finished: None,
      is_paused: false,
      last_paused: None,
      time_left: None,
    }
  }

  fn get_text_headers() -> Vec<String> {
    vec![
      "Title",
      "Started at",
      "Ended at",
      "Finished at",
      "Is paused",
      "Last time paused",
      "Time left",
    ]
    .iter()
    .map(|to_str| to_str.to_string())
    .collect()
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
    TimeStamp::push_text_date_time(&mut output, self.finished);

    output.push(if self.is_paused {
      "yes".to_string()
    } else {
      "no".to_string()
    });

    TimeStamp::push_text_date_time(&mut output, self.time_left);
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
    let seconds = TimeStamp::create_digit_with_at_least_2_digits(date.second());
    let minutes = TimeStamp::create_digit_with_at_least_2_digits(date.minute());
    let hours = TimeStamp::create_digit_with_at_least_2_digits(date.hour());
    let days = TimeStamp::create_digit_with_at_least_2_digits(date.day());
    let months = TimeStamp::create_digit_with_at_least_2_digits(date.month());

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

  fn create_digit_with_at_least_2_digits(digits: u32) -> String {
    if digits < 10 {
      format!("0{}", digits)
    } else {
      digits.to_string()
    }
  }
}
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn should_convert_to_str_vec() {
    let title = "To vec";
    let started = Utc.ymd(2014, 8, 24).and_hms(18, 8, 24);
    let actual_data = TimeStamp::with_started(title, started);

    let actual_vec = actual_data.to_str_vec();

    assert_eq!(
      actual_vec,
      vec![
        title.to_string(),
        "On 08.24.2014 at 18:08:24".to_string(),
        TimeStamp::NOT_AVAILABLE.to_string(),
        TimeStamp::NOT_AVAILABLE.to_string(),
        "no".to_string(),
        TimeStamp::NOT_AVAILABLE.to_string(),
        TimeStamp::NOT_AVAILABLE.to_string(),
      ]
    );
  }
  #[test]
  fn should_convert_to_right_format_from_data() {
    let time = Utc.ymd(2014, 7, 8).and_hms(22, 45, 21);

    let actual_text_format = TimeStamp::time_to_str(time);
    const EXPECTED: &str = "On 07.08.2014 at 22:45:21";
    assert_eq!(EXPECTED, actual_text_format);
  }

  #[test]
  fn should_return_table_for_time_stamps() {
    let input = vec![
      TimeStamp::with_started(
        "1. Line with more content",
        Utc.ymd(2018, 2, 1).and_hms(14, 12, 24),
      ),
      TimeStamp::with_started(
        "2. Line with more content",
        Utc.ymd(2022, 2, 1).and_hms(12, 32, 34),
      ),
    ];

    let actual_table = TimeStamp::create_text_table_from_time_stamps(&input);
    let expected = "Title                      Started at                 Ended at  Finished at  Is paused  Last time paused  Time left  
1. Line with more content  On 02.01.2018 at 14:12:24  N/A       N/A          no         N/A               N/A        
2. Line with more content  On 02.01.2022 at 12:32:34  N/A       N/A          no         N/A               N/A        
".to_string();
    for (expected_side, actual_side) in expected.lines().zip(actual_table.lines()) {
      assert_eq!(expected_side, actual_side);
    }
  }
}
