pub mod stop_watch;
pub mod time_entities_controller;
pub mod time_stamp;

trait TimeEntity {
  fn get_title(&self) -> &str;
}
