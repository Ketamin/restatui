mod app;
mod ui;

use crate::app::{App, ActiveField};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
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
                    KeyCode::Char('h') => {
                        app.active_field = match app.active_field {
                            ActiveField::Url => ActiveField::Search,
                            ActiveField::Response => ActiveField::History,
                            _ => app.active_field,
                        };
                    }
                    KeyCode::Char('l') => {
                        app.active_field = match app.active_field {
                            ActiveField::Search => ActiveField::Url,
                            ActiveField::History => ActiveField::Response,
                            _ => app.active_field,
                        };
                    }
                    KeyCode::Char('k') => {
                        app.active_field = match app.active_field {
                            ActiveField::History => ActiveField::Search,
                            ActiveField::Response => ActiveField::Url,
                            _ => app.active_field,
                        };
                    }
                    KeyCode::Char('j') => {
                        app.active_field = match app.active_field {
                            ActiveField::Search => ActiveField::History,
                            ActiveField::Url => ActiveField::Response,
                            _ => app.active_field,
                        };
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    
                    KeyCode::Char(c) => {
                        if matches!(app.active_field, ActiveField::Url) {
                            app.url_input.push(c);
                        } else if matches!(app.active_field, ActiveField::Search) {
                            app.search_input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if matches!(app.active_field, ActiveField::Url) {
                            app.url_input.pop();
                        } else if matches!(app.active_field, ActiveField::Search) {
                            app.search_input.pop();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
}