use crate::command::{BuiltInCommand, Command};

pub enum EvalResult {
    /// Result of evaluating a command - this just tells the REPL whether
    /// to continue or exit.
    Continue,
    Exit,
}

fn eval_exe(exe: &String, _args: &[String]) -> EvalResult {
    println!("{}: command not found", exe);
    EvalResult::Continue
}

/// Handle echo command - this function can assume we've already checked the command.
fn eval_echo(args: &[String]) -> EvalResult {
    let output = args.join(" ");
    println!("{}", output);
    EvalResult::Continue
}

fn eval_builtin(command: &BuiltInCommand) -> EvalResult {
    match command {
        BuiltInCommand::Echo { args } => eval_echo(args),
        BuiltInCommand::Type { .. } => todo!(),
        BuiltInCommand::Exit => EvalResult::Exit,
    }
}

pub fn eval_command(command: &Command) -> EvalResult {
    match command {
        Command::Empty => EvalResult::Continue,
        Command::BuiltIn(c) => eval_builtin(c),
        Command::Executable { exe, args } => eval_exe(exe, args),
    }
}
