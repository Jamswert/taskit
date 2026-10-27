use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub items: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub task: String,
    pub complete: bool,
    pub date_added: DateTime<Utc>
}