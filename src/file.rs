use crate::store::Tasks;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub fn task_path() -> PathBuf {
    PathBuf::from("taskit-tasks.json")
}

pub fn load() -> Result<Tasks, String> {
    let file_contents = match fs::read_to_string(task_path()) {
        Ok(contents) => contents,
        Err(msg) if msg.kind() == ErrorKind::NotFound => return Ok(Tasks { items: Vec::new() }),
        Err(msg) => return Err(msg.to_string()),
    };

    serde_json::from_str(&file_contents).map_err(|msg| msg.to_string())
}

pub fn save(tasks: &Tasks) -> Result<(), String> {
    let serialized = serde_json::to_string_pretty(tasks).map_err(|msg| msg.to_string())?;
    fs::write(task_path(), serialized).map_err(|msg| msg.to_string())?;

    Ok(())
}
