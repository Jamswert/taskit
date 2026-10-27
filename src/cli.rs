pub enum Command {
    Add { task: String },
    List,
    Help,
    Done { id: u32 },
    Remove { id: u32 },
}

pub fn parse(args: &[String]) -> Result<Command, String> {
    let Some((command, rest)) = args.split_first() else {
        return Err(String::from("Missing argument: command"));
    };

    match command.as_str() {
        "add" => {
            todo!()
        }
        "ls" | "list" => {
            todo!()
        }
        "help" => {
            todo!()
        }
        "done" => {
            todo!()
        }
        "rm" | "remove" => {
            todo!()
        }
        other => Err(format!("Unknown command {}", other)),
    }
}
