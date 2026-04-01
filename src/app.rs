#[derive(Clone, Copy)]
pub enum ActiveField {
    Search,
    History,
    Url,
    Response,
}

pub struct App {
    pub search_input: String,
    pub history_selection: usize,
    pub url_input: String,
    pub response_content: String,
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
