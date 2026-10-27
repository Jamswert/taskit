use std::fmt::Display;

pub enum Command {
    Add { task: String },
    List,
    Help,
    Done { id: u32 },
    Remove { id: u32 },
}

pub enum ParseError {
    UnknownCommand(String),
    MissingArgument(&'static str),
    NotImplemented(&'static str),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCommand(command) => write!(
                f,
                "Unknown command: '{command}'. Run 'taskit help' to see usage."
            ),
            Self::MissingArgument(argument) => write!(
                f,
                "Missing argument: '{argument}'. Run 'taskit help' to see usage."
            ),
            Self::NotImplemented(command) => {
                write!(f, "Command: '{command}' is not yet implemented.")
            }
        }
    }
}

pub fn parse(args: &[String]) -> Result<Command, ParseError> {
    let Some((command, rest)) = args.split_first() else {
        return Err(ParseError::MissingArgument("command"));
    };

    match command.as_str() {
        "add" => Err(ParseError::NotImplemented("add")),
        "ls" | "list" => Err(ParseError::NotImplemented("list")),
        "help" => Err(ParseError::NotImplemented("help")),
        "done" => Err(ParseError::NotImplemented("done")),
        "rm" | "remove" => Err(ParseError::NotImplemented("remove")),
        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}
