use std::cmp::max;
/// Formats vectors of string into one text in which every column has the
/// same width.
/// # Panics
/// Assumes that every inner string vector has the same length
/// # Example
/// ```
/// use stamp_member::format_utils::format_to_text_table;
///
/// let test_data = [
///   vec!["X".repeat(10), "Y".repeat(20)],
///   vec!["X".repeat(2), "t".repeat(5)],
/// ];
/// let table = format_to_text_table(&test_data, 2);
/// let expected = "XXXXXXXXXX  YYYYYYYYYYYYYYYYYYYY  
/// XX          ttttt                 \n";
/// assert_eq!(table, expected);
///
/// // Output with println!(table);
/// //XXXXXXXXXX  YYYYYYYYYYYYYYYYYYYY
/// //XX          ttttt
/// ```
pub fn format_to_text_table(to_format: &[Vec<String>], rim_spaces: usize) -> String {
  let mut column_width_vec = calc_max_column_width_for(to_format);

  let mut table =
    prepare_empty_table_enough_capacity(&mut column_width_vec, rim_spaces, to_format.len());
  create_table_with_padding(&mut table, to_format, &column_width_vec);

  table
}

fn create_table_with_padding(
  table: &mut String,
  to_format: &[Vec<String>],
  column_width_vec: &Vec<usize>,
) {
  for row in to_format {
    for (column, content) in row.iter().enumerate() {
      let current_column_len = content.chars().count();
      let width_pad_to = column_width_vec[column];
      let spaces_to_add = width_pad_to - current_column_len;
      let padding = " ".repeat(spaces_to_add);

      table.push_str(content);
      table.push_str(&padding);
    }
    table.push('\n');
  }
}
/// Adds rim_spaces as number to every row of given vec to account for spacing to right for a column.
/// Creates empty string for enough capacity so no reallocation on heap is needed later.
fn prepare_empty_table_enough_capacity(
  column_width_vec: &mut Vec<usize>,
  rim_spaces: usize,
  number_rows: usize,
) -> String {
  let mut size_of_one_row: usize = 0;

  for column_width in column_width_vec.iter_mut() {
    *column_width += rim_spaces;
    size_of_one_row += *column_width;
  }

  let size_with_newlines = size_of_one_row + 1;
  let size_of_all_rows = size_with_newlines * number_rows;

  String::with_capacity(size_of_all_rows)
}
/// Calculates a vec with same length as a row of to_format 2d vec. Each index of the returned vec
/// contains the max column width of the respective columns of every row of to_format vec param.
/// # Errors
/// Assumes every row of param to_format has the same len, number of columns.
/// # Example
/// to_param with layout:
/// ```text
/// [
///   [ "12345", "123", "1234" ],
///   [ "12", "123456", "12" ]
/// ]
/// ```
/// Returns
/// ```text
/// [ 5, 6, 4 ]
/// ```
fn calc_max_column_width_for(to_format: &[Vec<String>]) -> Vec<usize> {
  let number_columns = to_format[0].len();
  let mut max_width: Vec<usize> = vec![0usize; number_columns];
  for index in 0..number_columns {
    for row in to_format {
      debug_assert_eq!(
        number_columns,
        row.len(),
        "One row is not as long as the others."
      );
      let column_width = row[index].chars().count();
      max_width[index] = max(column_width, max_width[index]);
    }
  }

  max_width
}

pub fn with_at_least_2_digits(digits: u64) -> String {
  format!("{:02}", digits)
}

pub fn with_at_least_3_digits(digits: u64) -> String {
  format!("{:03}", digits)
}

pub fn with_at_least_4_digits(digits: u64) -> String {
  format!("{:04}", digits)
}

#[cfg(test)]
mod testing {
  use super::*;

  #[test]
  fn should_calculate_max_column_width() {
    let input = [
      to_string_vec(&["12345", "123", "1234"]),
      to_string_vec(&["12", "123456", "12"]),
    ];

    let actual = calc_max_column_width_for(&input);

    assert_eq!(actual, vec![5, 6, 4])
  }

  fn to_string_vec(to_convert: &[&str]) -> Vec<String> {
    to_convert
      .iter()
      .map(|to_turn| to_turn.to_string())
      .collect()
  }

  #[test]
  fn should_preallocate_enough_space_for_string() {
    let mut columns = vec![5, 6, 4];
    let rim_spaces = 2;
    let row_count = 3;

    let expected_columns = vec![7, 8, 6];
    // + row_count for \n at the end of a row.
    let expected_capacity: usize = (expected_columns.iter().sum::<usize>() * row_count) + row_count;

    let actual = prepare_empty_table_enough_capacity(&mut columns, rim_spaces, row_count);

    assert_eq!(
      columns, expected_columns,
      "Did not add rim_spaces {rim_spaces}"
    );
    assert_eq!(expected_capacity, actual.capacity());
  }
}
