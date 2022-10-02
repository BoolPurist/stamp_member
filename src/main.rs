#[allow(dead_code)]
use stamp_member::format_utils::format_to_text_table;
fn main() {
  let test_data = [
    vec!["X".repeat(10), "Y".repeat(20)],
    vec!["X".repeat(2), "t".repeat(5)],
  ];
  let table = format_to_text_table(&test_data, 2);
  let expected = "XXXXXXXXXX  YYYYYYYYYYYYYYYYYYYY  
XX          ttttt                 \n";
  assert_eq!(table, expected);
  println!("{}", expected);
  // let provided_args: Vec<String> = std::env::args().skip(1).collect();
}
