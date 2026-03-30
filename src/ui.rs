use ratatui::{
    style::{ Color, Modifier, Style },
    layout::{ Constraint, Layout, Rect },
    widgets::{ Block, BorderType, Paragraph },
    Frame,
};
use crate::app::{ ActiveField, App }; // import App from neighboring module

const LEFT_SECTION_WIDTH: u16 = 60;

fn get_pane_title_style(is_active: bool) -> Style {
    if is_active { Style::default().add_modifier(Modifier::BOLD) } else { Style::default() }
}

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
        Constraint::Fill(1),
    ]);

    let [left_section, right_section] = main_layout.areas(area);

    render_left(frame, left_section, app);
    render_right(frame, right_section, app);
}

fn render_titled_pane(frame: &mut Frame, area: Rect, title: &str, content: &str, is_active: bool) {
    let (border_type, border_style) = get_border_style(is_active);
    let title_style = get_pane_title_style(is_active);

    let padded_title = format!(" {} ", title);

    frame.render_widget(
        Paragraph::new(content).block(
            Block::bordered()
                .border_type(border_type)
                .style(border_style)
                .title(padded_title)
                .title_style(title_style)
        ),
        area
    );
}

pub fn render_left(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);

    let is_url_active = matches!(app.active_field, ActiveField::Search);

    render_titled_pane(frame, chunks[0], "Search", app.search_input.as_str(), is_url_active);

    let is_response_active = matches!(app.active_field, ActiveField::History);

    render_titled_pane(
        frame,
        chunks[1],
        "History",
        app.history_selection.to_string().as_str(),
        is_response_active
    )
}

pub fn render_right(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);

    let is_response_active = matches!(app.active_field, ActiveField::Url);

    render_titled_pane(frame, chunks[0], "URL", app.url_input.as_str(), is_response_active);

    let is_response_active = matches!(app.active_field, ActiveField::Response);

    render_titled_pane(
        frame,
        chunks[1],
        "Response",
        app.response_content.as_str(),
        is_response_active
    )
}
