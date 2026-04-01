mod app;
mod ui;
mod components;

use crate::app::{ App, ActiveField };
use crossterm::event::{ self, Event, KeyCode, KeyEventKind, KeyModifiers };
use ratatui::DefaultTerminal;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('h') => app.move_left(),
                        KeyCode::Char('l') => app.move_right(),
                        KeyCode::Char('k') => app.move_up(),
                        KeyCode::Char('j') => app.move_down(),
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char(c) =>
                            match app.active_field {
                                ActiveField::Url => app.url_input.push(c),
                                ActiveField::Search => app.search_input.push(c),
                                _ => {}
                            }
                        KeyCode::Backspace =>
                            match app.active_field {
                                ActiveField::Url => {
                                    app.url_input.pop();
                                }
                                ActiveField::Search => {
                                    app.search_input.pop();
                                }
                                _ => {}
                            }
                        _ => {}
                    }
                }
            }
        }
    }
}
