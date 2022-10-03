#[allow(dead_code)]
use stamp_member::time_stamp::TimeStamp;
use stamp_member::{DataSourceJsonError, TimeStampSource};
fn main() {
  let provider = TimeStampSource::new("/home/nice_graphic");
}

fn serde_with_vec() {
  let data = vec![
    TimeStamp::new("Hello"),
    TimeStamp::new("Bye"),
    TimeStamp::new("World"),
  ];
  let text = serde_json::to_string(&data).unwrap();
  println!("Json: {text}\n");
  let data_back: Vec<TimeStamp> = serde_json::from_str(&text).unwrap();
  println!("Data: {:?}", &data_back);
}
