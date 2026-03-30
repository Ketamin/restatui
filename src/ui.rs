use ratatui::{
    style::{Color, Modifier, Style},
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use crate::app::{ActiveField, App}; // import App from neighboring module

const LEFT_SECTION_WIDTH: u16 = 60;

fn get_border_style(is_active: bool) -> (BorderType, Style) {
    if is_active {
        (BorderType::Thick, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    } else {
        (BorderType::Rounded, Style::default().fg(Color::DarkGray))
    }
}

pub fn render(frame: &mut Frame, app: &App) {
  let area = frame.area();

  let main_layout = Layout::horizontal([
    Constraint::Length(LEFT_SECTION_WIDTH),
    Constraint::Fill(1)
  ]);

  let [left_section, right_section] = main_layout.areas(area);

  render_left(frame, left_section, app);
  render_right(frame, right_section, app);
}

pub fn render_left(frame: &mut Frame, area: Rect, app: &App) {
  let chunks = Layout::vertical([
    Constraint::Length(3),
    Constraint::Fill(1)
  ]).split(area);

  let is_url_active = matches!(app.active_field, ActiveField::Search);
  let (border_type, border_style) = get_border_style(is_url_active);

  frame.render_widget(
    Paragraph::new(app.search_input.as_str()).block(Block::bordered().border_type(border_type).style(border_style)
    .title("Search")),
    chunks[0]
  );

  let is_response_active = matches!(app.active_field, ActiveField::History);
  let (border_type, border_style) = get_border_style(is_response_active);

  frame.render_widget(
    Paragraph::new(app.history_selection.to_string().as_str()).block(Block::bordered().border_type(border_type).style(border_style).title("History")),
    chunks[1]
  )
}

pub fn render_right(frame: &mut Frame, area: Rect, app: &App) {
  let chunks = Layout::vertical([
    Constraint::Length(3),
    Constraint::Fill(1)
  ]).split(area);

  let is_response_active = matches!(app.active_field, ActiveField::Url);
  let (border_type, border_style) = get_border_style(is_response_active);

  frame.render_widget(
    Paragraph::new(app.url_input.as_str()).block(Block::bordered().border_type(border_type).style(border_style).title("URL")),
    chunks[0]
  );  

  let is_response_active = matches!(app.active_field, ActiveField::Response);
  let (border_type, border_style) = get_border_style(is_response_active);

  frame.render_widget(
    Paragraph::new(app.response_content.as_str()).block(Block::bordered().border_type(border_type).style(border_style).title("Response")),
    chunks[1]
  )
}