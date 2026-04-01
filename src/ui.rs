use ratatui::{ layout::{ Constraint, Layout, Rect }, Frame };
use crate::{ app::{ App } };
use crate::components::{ Component, Search, History, Url, Response };

const LEFT_SECTION_WIDTH: u16 = 60;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::horizontal([
        Constraint::Length(LEFT_SECTION_WIDTH),
        Constraint::Fill(1),
    ]).split(frame.area());

    render_left_column(frame, chunks[0], app);
    render_right_column(frame, chunks[1], app);
}

fn render_left_column(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);
    Search.render(frame, chunks[0], app);
    History.render(frame, chunks[1], app);
}

fn render_right_column(frame: &mut Frame, area: Rect, app: &App) {
    let vertical_chunks = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(
        area
    );

    // Url now handles the MethodSelector internally
    Url.render(frame, vertical_chunks[0], app);
    Response.render(frame, vertical_chunks[1], app);
}
