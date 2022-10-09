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
  let mut column_width_vec = calc_max_column_with_for(to_format);

  let mut size_of_all_rows: usize = 0;
  for column_width in column_width_vec.iter_mut() {
    *column_width += rim_spaces;
    size_of_all_rows += *column_width;
  }

  let size_with_newlines = size_of_all_rows + column_width_vec.len();
  let mut table = String::with_capacity(size_with_newlines);

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

fn calc_max_column_with_for(to_format: &[Vec<String>]) -> Vec<usize> {
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
