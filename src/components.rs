use ratatui::{
    Frame,
    layout::{ Constraint, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Line, Span },
    widgets::{ Block, BorderType, Paragraph },
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

pub struct MethodSelector;
impl Component for MethodSelector {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        // Only styling the text here
        let method_color = app.http_method.get_color();
        let is_active = matches!(app.active_field, ActiveField::Url);

        let method_text = Span::styled(
            app.http_method.as_str(),
            Style::default()
                .fg(method_color)
                .add_modifier(if is_active { Modifier::BOLD } else { Modifier::empty() })
        );

        // Render the text inside the small area we gave it
        frame.render_widget(Paragraph::new(Line::from(vec![Span::raw(" "), method_text])), area);
    }
}

pub struct Url;
impl Component for Url {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        let is_active = matches!(app.active_field, ActiveField::Url);

        // Create the outer block
        let block = Block::bordered()
            .title(" URL ")
            .border_type(if is_active { BorderType::Thick } else { BorderType::Rounded })
            .style(if is_active { Style::default().fg(Color::Yellow) } else { Style::default() });

        // Get the area inside the borders
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Split the inner area horizontally
        let chunks = Layout::horizontal([
            Constraint::Length(6), // Space for MethodSelector
            Constraint::Fill(1), // Space for the URL text
        ]).split(inner_area);

        // Delegate to MethodSelector
        MethodSelector.render(frame, chunks[0], app);

        // Render the separator and URL input manually
        frame.render_widget(Paragraph::new(app.url_input.as_str()), chunks[1]);
    }
}

pub struct Response;
impl Component for Response {
    fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        let active = matches!(app.active_field, ActiveField::Response);
        TitledPane::render(frame, area, "Response", &app.response_content, active);
    }
}
