use std::ops::{Add, Sub};

use chrono::{DateTime, Duration, Utc};

use crate::{format_utils, return_if_with};
#[derive(Default, PartialEq, Debug)]
pub struct DateDifference {
  seconds: u64,
  minutes: u64,
  hours: u64,
  days: u64,
  years: u64,
}

impl DateDifference {
  pub fn new(total_secs: u64) -> Self {
    let mut result: DateDifference = Default::default();
    let mut left_total_secs = total_secs;

    extract_advance_time_unit(&mut result.seconds, &mut left_total_secs, 60);
    return_if_with!(left_total_secs, 0, result);

    extract_advance_time_unit(&mut result.minutes, &mut left_total_secs, 60);
    return_if_with!(left_total_secs, 0, result);

    extract_advance_time_unit(&mut result.hours, &mut left_total_secs, 24);
    return_if_with!(left_total_secs, 0, result);

    extract_advance_time_unit(&mut result.days, &mut left_total_secs, 365);

    result.years = left_total_secs;

    return result;

    fn extract_advance_time_unit(dest: &mut u64, t_secs: &mut u64, unit: u64) {
      *dest = *t_secs % unit;
      *t_secs /= unit;
    }
  }
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
#[allow(dead_code)]
/// TODO: remove dead code
fn calc_date_moment_difference(later: &DateTime<Utc>, earlier: &DateTime<Utc>) -> DateDifference {
  debug_assert!(
    earlier <= later,
    "1. param should be greater, later in time, than the 2. one."
  );

  let difference: Duration = later.sub(earlier.clone());

  debug_assert!(
    difference.num_seconds() >= 0,
    "Total seconds should be positive because 1. param should be greater than the 2."
  );

  DateDifference::new(difference.num_seconds() as u64)
}

pub fn duration_with_hms(hours: i64, minutes: i64, secs: i64) -> chrono::Duration {
  chrono::Duration::hours(hours)
    .add(Duration::minutes(minutes))
    .add(Duration::seconds(secs))
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
