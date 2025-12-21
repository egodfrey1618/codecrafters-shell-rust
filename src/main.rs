use std::io;
use std::io::Write;

struct Command {
    command: String,
}

fn read_command() -> Result<Command, io::Error> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    let without_newline = s.trim().to_string();

    Ok(Command {
        command: without_newline,
    })
}

fn print_prompt() -> Result<(), io::Error> {
    print!("$ ");
    io::stdout().flush()
}

fn eval_command(command: &Command) {
    println!("{}: command not found", command.command);
}

fn main() {
    loop {
        print_prompt().expect("IO error when printing prompt");
        let command = read_command().expect("IO error when reading command");
        eval_command(&command);
    }
}
