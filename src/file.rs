use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs;
use crate::store::Tasks;

pub fn task_path() -> PathBuf {
    PathBuf::from("taskit-tasks.json")
}

pub fn load() -> Result<Tasks, String> {
    let file_contents = match fs::read_to_string(task_path()) {
        Ok(contents) => contents,
        Err(msg) if msg.kind() == ErrorKind::NotFound => {
            return Ok(Tasks { items: Vec::new() })
        },
        Err(msg) => return Err(msg.to_string())
    };

    serde_json::from_str(&file_contents).map_err(|msg| msg.to_string())
}