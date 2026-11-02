mod cli;
mod file;
mod store;

use crate::cli::{Command, parse};
use crate::file::{load, save};
use crate::store::Item;

use chrono::Utc;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let command = match parse(&args) {
        Ok(command) => command,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    match run(command) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

fn run(command: Command) -> Result<(), String> {
    let mut tasks = load()?;

    match command {
        Command::Add { task } => {
            let id = tasks.next_id();
            let date_time_added = Utc::now();
            let complete = false;

            let new_item = Item {
                id,
                task,
                complete,
                date_added: date_time_added,
            };

            tasks.items.push(new_item);

            return match save(&tasks) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to save tasks: {e}")),
            };
        }
        Command::List => {}
        Command::Help => {}
        Command::Done { id } => {}
        Command::Remove { id } => {}
    }

    Ok(())
}
