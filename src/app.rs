pub enum ActiveField {
  // Left pane
  Search,
  History,
  // Right pane
  Url,
  Response,
}

pub struct App {
  // Left pane
  pub search_input: String,
  pub history_selection: usize,
  // Right pane
  pub url_input: String,
  pub response_content: String,
  // Others
  pub active_field: ActiveField,
}

impl App {
  pub fn new() -> Self {
    Self {
      search_input: String::new(),
      history_selection: 0,
      url_input: String::new(),
      response_content: String::new(),
      active_field: ActiveField::Search,
    }
  }
}