use std::ops::Sub;

use chrono::{DateTime, Duration, Utc};

use crate::format_utils;
#[derive(Default, PartialEq, Debug)]
pub struct DateDifference {
  seconds: u64,
  minutes: u64,
  hours: u64,
  days: u64,
  years: u64,
}

impl ToString for DateDifference {
  fn to_string(&self) -> String {
    let seconds = format_utils::with_at_least_2_digits(self.seconds);
    let minutes = format_utils::with_at_least_2_digits(self.minutes);
    let hours = format_utils::with_at_least_2_digits(self.hours);

    let days = format_utils::with_at_least_3_digits(self.days);
    let years = format_utils::with_at_least_4_digits(self.years);

    format!("{years}:{days} {hours}:{minutes}:{seconds}")
  }
}
fn calc_date_moment_difference(greater: &DateTime<Utc>, smaller: &DateTime<Utc>) -> DateDifference {
  debug_assert!(smaller <= greater);

  let difference: Duration = greater.clone().sub(smaller.clone());

  debug_assert!(difference.num_seconds() >= 0);

  let mut total_secs: u64 = difference.num_seconds() as u64;

  let mut result: DateDifference = Default::default();
  result.seconds = total_secs % 60;
  total_secs /= 60;
  if total_secs == 0 {
    return result;
  }

  result.minutes = total_secs % 60;
  total_secs /= 60;
  if total_secs == 0 {
    return result;
  }

  result.hours = total_secs % 24;
  total_secs /= 24;
  if total_secs == 0 {
    return result;
  }

  result.days = total_secs % 365;
  total_secs /= 365;

  result.years = total_secs;

  result
}
#[cfg(test)]
mod tests {
  use chrono::TimeZone;

  use super::*;

  #[test]
  fn should_have_seconds_difference() {
    let greater = Utc.ymd(2000, 1, 1).and_hms(1, 0, 25);
    let smaller = Utc.ymd(2000, 1, 1).and_hms(1, 0, 12);
    let default = Default::default();
    let expected = DateDifference {
      seconds: 13,
      ..default
    };

    let actual = calc_date_moment_difference(&greater, &smaller);

    assert_eq!(expected, actual);
  }

  #[test]
  fn should_have_secs_min_difference() {
    let greater = Utc.ymd(2000, 1, 1).and_hms(1, 2, 35);
    let smaller = Utc.ymd(2000, 1, 1).and_hms(1, 0, 12);
    let default = Default::default();
    let expected = DateDifference {
      seconds: 23,
      minutes: 2,
      ..default
    };

    let actual = calc_date_moment_difference(&greater, &smaller);

    assert_eq!(expected, actual);
  }

  #[test]
  fn should_have_secs_min_hour_difference() {
    let greater = Utc.ymd(2000, 1, 1).and_hms(8, 2, 35);
    let smaller = Utc.ymd(2000, 1, 1).and_hms(1, 23, 0);
    let default = Default::default();
    let expected = DateDifference {
      seconds: 35,
      minutes: 39,
      hours: 6,
      ..default
    };

    let actual = calc_date_moment_difference(&greater, &smaller);

    assert_eq!(expected, actual);
  }

  #[test]
  fn should_have_secs_min_hour_day_year_difference() {
    let greater = Utc.ymd(2000, 5, 28).and_hms(13, 7, 12);
    let smaller = Utc.ymd(1998, 3, 11).and_hms(2, 31, 5);
    let expected = DateDifference {
      seconds: 7,
      minutes: 36,
      hours: 10,
      days: 79,
      years: 2,
    };

    let actual = calc_date_moment_difference(&greater, &smaller);

    assert_eq!(expected, actual);
  }

  #[test]
  fn should_to_string_date_difference() {
    let to_convert = DateDifference {
      seconds: 7,
      minutes: 36,
      hours: 10,
      days: 79,
      years: 2,
    };

    let text = to_convert.to_string();
    let expected = "0002:079 10:36:07";
    assert_eq!(expected, text);
  }
}
