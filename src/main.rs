use std::io;
use std::io::{Write};

enum Command {
    Command(String),
}

fn read_command() -> Result<Command, io::Error> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    Ok(Command::Command(s))
}

fn main() {
    print!("$ ");
    io::stdout().flush().expect("Error when flushing stdout");
    let command = read_command().expect("IO error when reading command");

    match command {
        Command::Command(s) => print!("Unknown command: {s}"),
    }
}
