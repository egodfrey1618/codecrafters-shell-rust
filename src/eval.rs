use crate::command::{BuiltInCommand, Command};

pub enum EvalResult {
    /// Result of evaluating a command - this just tells the REPL whether
    /// to continue or exit.
    Continue,
    Exit,
}

fn eval_exe(exe: &String, _args: &[String]) -> EvalResult {
    println!("{exe}: command not found");
    EvalResult::Continue
}

fn eval_echo(args: &[String]) -> EvalResult {
    let output = args.join(" ");
    println!("{output}");
    EvalResult::Continue
}

fn eval_type(inner_command: &Command) -> EvalResult {
    match inner_command {
        Command::BuiltIn(built_in) => {
            let name = built_in.to_variant_str();
            println!("{name} is a shell builtin");
        }
        Command::Executable { exe, args: _ } => {
            println!("{exe}: not found")
        }
        Command::Empty => (),
    };
    EvalResult::Continue
}

fn eval_builtin(command: &BuiltInCommand) -> EvalResult {
    match command {
        BuiltInCommand::Echo { args } => eval_echo(args),
        BuiltInCommand::Type { command } => eval_type(command),
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
