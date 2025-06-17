use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub name: String,
    pub comment: Option<String>,
    pub exec: Option<String>,
    pub icon_path: Option<String>,
}

impl App {
    pub fn new(name: String) -> Self {
        Self {
            name,
            comment: None,
            exec: None,
            icon_path: None,
        }
    }

    pub fn with_comment(mut self, comment: Option<String>) -> Self {
        self.comment = comment;
        self
    }

    pub fn with_exec(mut self, exec: Option<String>) -> Self {
        self.exec = exec;
        self
    }

    pub fn with_icon_path(mut self, icon_path: Option<String>) -> Self {
        self.icon_path = icon_path;
        self
    }
} 