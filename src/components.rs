use ratatui::{
    layout::Rect,
    style::{ Color, Modifier, Style },
    widgets::{ Block, BorderType, Paragraph },
    Frame,
};
use crate::app::{ ActiveField, App };

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App);
}

pub struct TitledPane;

impl TitledPane {
    pub fn render(frame: &mut Frame, area: Rect, title: &str, content: &str, is_active: bool) {
        let style = if is_active {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::bordered()
            .border_type(if is_active { BorderType::Thick } else { BorderType::Rounded })
            .style(style)
            .title(format!(" {} ", title))
            .title_style(
                if is_active {
                    Style::default().add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                }
            );

        frame.render_widget(Paragraph::new(content).block(block), area);
    }
}

pub struct Search;
impl Component for Search {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        let active = matches!(app.active_field, ActiveField::Search);
        TitledPane::render(frame, area, "Search", &app.search_input, active);
    }
}

pub struct History;
impl Component for History {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        let active = matches!(app.active_field, ActiveField::History);
        TitledPane::render(frame, area, "History", &app.history_selection.to_string(), active);
    }
}

pub struct Url;
impl Component for Url {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        let active = matches!(app.active_field, ActiveField::Url);
        TitledPane::render(frame, area, "URL", &app.url_input, active);
    }
}

pub struct Response;
impl Component for Response {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        let active = matches!(app.active_field, ActiveField::Response);
        TitledPane::render(frame, area, "Response", &app.response_content, active);
    }
}
