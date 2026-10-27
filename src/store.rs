use chrono::{DateTime, Utc};

pub struct Tasks {
    pub items: Vec<Item>
}

pub struct Item {
    pub task: String,
    pub complete: bool,
    pub date_added: DateTime<Utc>
}