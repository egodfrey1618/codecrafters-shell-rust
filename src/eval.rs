use crate::command::Command;

pub enum EvalResult {
    /// Result of evaluating a command - this just tells the REPL whether
    /// to continue or exit.
    Continue,
    Exit,
}

pub fn handle_unknown_command(command: &Command) {
    println!("{}: command not found", command.command)
}

/// Handle echo command - this function can assume we've already checked the command.
pub fn eval_echo(command: &Command) {
    let output = command.args.join(" ");
    println!("{}", output);
}

pub fn eval_command(command: &Command) -> EvalResult {
    match command.command.as_str() {
        // Built-ins
        "exit" => EvalResult::Exit,
        "" => EvalResult::Continue,
        "echo" => {
            eval_echo(command);
            EvalResult::Continue
        }
        // Anything else.
        _ => {
            handle_unknown_command(command);
            EvalResult::Continue
        }
    }
}
