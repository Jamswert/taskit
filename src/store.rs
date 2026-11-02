use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub items: Vec<Item>,
}

impl Tasks {
    pub fn next_id(&self) -> u32 {
        self.items.iter().map(|item| item.id).max().unwrap_or(0) + 1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: u32,
    pub task: String,
    pub complete: bool,
    pub date_added: DateTime<Utc>,
}
