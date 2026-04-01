#[derive(Clone, Copy)]
pub enum ActiveField {
    Search,
    History,
    Url,
    Response,
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
        }
    }

    pub fn next(&mut self) {
        *self = match self {
            HttpMethod::GET => HttpMethod::POST,
            HttpMethod::POST => HttpMethod::PUT,
            HttpMethod::PUT => HttpMethod::DELETE,
            HttpMethod::DELETE => HttpMethod::PATCH,
            HttpMethod::PATCH => HttpMethod::GET,
        };
    }

    pub fn get_color(&self) -> ratatui::style::Color {
        match self {
            HttpMethod::GET => ratatui::style::Color::Green,
            HttpMethod::POST => ratatui::style::Color::Blue,
            HttpMethod::PUT => ratatui::style::Color::Yellow,
            HttpMethod::DELETE => ratatui::style::Color::Red,
            HttpMethod::PATCH => ratatui::style::Color::Magenta,
        }
    }
}

pub struct App {
    pub search_input: String,
    pub history_selection: usize,
    pub url_input: String,
    pub response_content: String,
    pub active_field: ActiveField,
    pub http_method: HttpMethod,
}

impl App {
    pub fn new() -> Self {
        Self {
            search_input: String::new(),
            history_selection: 0,
            url_input: String::new(),
            response_content: String::new(),
            active_field: ActiveField::Search,
            http_method: HttpMethod::GET,
        }
    }

    pub fn move_right(&mut self) {
        self.active_field = match self.active_field {
            ActiveField::Search => ActiveField::Url,
            ActiveField::History => ActiveField::Response,
            _ => self.active_field,
        };
    }

    pub fn move_left(&mut self) {
        self.active_field = match self.active_field {
            ActiveField::Url => ActiveField::Search,
            ActiveField::Response => ActiveField::History,
            _ => self.active_field,
        };
    }

    pub fn move_up(&mut self) {
        self.active_field = match self.active_field {
            ActiveField::History => ActiveField::Search,
            ActiveField::Response => ActiveField::Url,
            _ => self.active_field,
        };
    }

    pub fn move_down(&mut self) {
        self.active_field = match self.active_field {
            ActiveField::Search => ActiveField::History,
            ActiveField::Url => ActiveField::Response,
            _ => self.active_field,
        };
    }
}
