mod command;
mod eval;
use crate::command::Command;
use crate::eval::{EvalResult, eval_command};
use std::io;
use std::io::Write;

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

fn main() {
    loop {
        print_prompt().expect("IO error when printing prompt");
        let command = read_command().expect("IO error when reading command");
        match eval_command(&command) {
            EvalResult::Continue => (),
            EvalResult::Exit => {
                break;
            }
        }
    }
}
