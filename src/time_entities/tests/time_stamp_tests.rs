use std::ops::Add;

use chrono::Duration;

use super::*;

#[test]
fn should_finish_on_2_hours_later() {
  let (mut actual_data, expected_ended) = setup_finish();

  match actual_data.finish() {
    Ok(ended_time) => assert_eq!(&expected_ended, ended_time),
    Err(_) => panic!("Failure on finished time stamp"),
  };
}

#[test]
fn should_return_error_on_already_finished() {
  // Set up
  let (mut actual_data, expected_ended) = setup_finish();
  actual_data.finish().unwrap();
  // Change current time and see if ended time remains the same
  actual_data.set_new(&expected_ended.add(Duration::hours(2)));

  // Act
  match actual_data.finish() {
    // Assert
    Ok(_) => panic!("Should return error on already ended time stamp"),
    Err(actual_time) => assert_eq!(
      &expected_ended, actual_time,
      "Should still have the same ended time"
    ),
  }
}

fn setup_finish() -> (TimeStamp, DateTime<Utc>) {
  let started = Utc.ymd(2012, 8, 8).and_hms(2, 2, 2);
  let mut actual_data = TimeStamp::with_started("2 Hours later ...", started.clone());
  let expected_ended = started.add(Duration::hours(2));
  actual_data.set_new(&expected_ended);

  (actual_data, expected_ended)
}

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
  let expected =
    "Title                      Started at                 Ended at  Is paused  Last time paused  
1. Line with more content  On 02.01.2018 at 14:12:24  N/A       no         N/A               
2. Line with more content  On 02.01.2022 at 12:32:34  N/A       no         N/A               
"
    .to_string();
  for (expected_side, actual_side) in expected.lines().zip(actual_table.lines()) {
    assert_eq!(expected_side, actual_side);
  }
}
