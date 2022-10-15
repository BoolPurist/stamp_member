#[macro_export]
macro_rules! return_if_with {
  ($to_test:ident, $cond:expr, $ret_val:expr) => {
    if ($to_test == $cond) {
      return $ret_val;
    }
  };
}
