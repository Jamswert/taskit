mod cli;
mod store;

use cli::parse;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match parse(&args) {
        Ok(command) => {
            println!("Do command");
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
}
