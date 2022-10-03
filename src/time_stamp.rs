use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeStamp {
  title: String,
  started: DateTime<Utc>,
  ended: Option<DateTime<Utc>>,
  is_paused: bool,
  time_left: Option<usize>,
}

impl TimeStamp {
  const NOT_AVAILABLE: &'static str = "N/A";
  pub fn new(title: &str) -> TimeStamp {
    TimeStamp {
      title: title.to_string(),
      started: Utc::now(),
      ended: None,
      is_paused: false,
      time_left: None,
    }
  }

  pub fn new_with_started(title: &str, started: DateTime<Utc>) -> TimeStamp {
    TimeStamp {
      title: title.to_string(),
      started,
      ended: None,
      is_paused: false,
      time_left: None,
    }
  }

  /// Outputs a vector with text row with columns for:
  /// - Title: Label of timestamp/stopwatch.
  /// - Started: Time at which it was started.
  /// - Ended: Time at wich the stopwatch has ended or the timestamp was finished.
  /// N/A if it was not ended yet
  /// - Is paused: yes for stopped. no if not stopped.
  /// - Time left: Seconds left until stopwatch is done. N/A if not stopwatch
  /// # Example
  /// ```
  /// use chrono::Utc;
  /// use stamp_member::time_stamp;
  /// use stamp_member::time_stamp::TimeStamp;
  /// use chrono::TimeZone;
  ///
  ///let title = "To vec";
  ///let started = Utc.ymd(2004, 11, 18).and_hms(23, 8, 24);
  ///let actual_data = TimeStamp::new_with_started(title, started);
  ///
  ///let actual_vec = actual_data.to_str_vec();
  ///
  ///assert_eq!(
  ///  actual_vec,
  ///  vec![
  ///    title.to_string(),
  ///    "On 11.18.2004 at 23:08:24".to_string(),
  ///    "N/A".to_string(),
  ///    "no".to_string(),
  ///    "N/A".to_string(),
  ///  ]
  ///);
  /// ```
  pub fn to_str_vec(self) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    output.push(self.title);
    output.push(TimeStamp::time_to_str(self.started));

    if let Some(time) = self.ended {
      output.push(TimeStamp::time_to_str(time))
    } else {
      output.push(TimeStamp::NOT_AVAILABLE.to_string())
    }

    output.push(if self.is_paused {
      "yes".to_string()
    } else {
      "no".to_string()
    });

    if let Some(left_time) = self.time_left {
      output.push(left_time.to_string())
    } else {
      output.push(TimeStamp::NOT_AVAILABLE.to_string())
    }

    output
  }

  /// Returns an easy to read text presentation of date time.
  /// The date is shown as date, (month, day, year) and
  /// then time (hour minutes seconds) from left to right.
  /// # Example
  /// ```
  /// use chrono::Utc;
  /// use stamp_member::time_stamp;
  /// use stamp_member::time_stamp::TimeStamp;
  /// use chrono::TimeZone;
  ///
  ///let time = Utc.ymd(2018, 9, 2).and_hms(8, 5, 2);
  ///let actual_text_format = TimeStamp::time_to_str(time);
  ///
  ///const EXPECTED: &str = "On 09.02.2018 at 08:05:02";
  ///assert_eq!(EXPECTED, actual_text_format);
  /// ```
  pub fn time_to_str(date: DateTime<Utc>) -> String {
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
    let actual_data = TimeStamp::new_with_started(title, started);

    let actual_vec = actual_data.to_str_vec();

    assert_eq!(
      actual_vec,
      vec![
        title.to_string(),
        "On 08.24.2014 at 18:08:24".to_string(),
        TimeStamp::NOT_AVAILABLE.to_string(),
        "no".to_string(),
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
}
