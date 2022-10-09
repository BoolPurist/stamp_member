use clap::Parser;
#[allow(dead_code)]
use stamp_member::time_stamp::TimeStamp;
use stamp_member::{args_parser::AppCommand, data_access::get_data_path};
fn main() {
  let _args = AppCommand::parse();
  let path = get_data_path();
}
#[allow(dead_code)]
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
