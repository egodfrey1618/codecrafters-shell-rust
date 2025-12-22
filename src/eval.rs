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

pub fn eval_command(command: &Command) -> EvalResult {
    match command.command.as_str() {
        // Built-ins
        "exit" => EvalResult::Exit,
        // Anything else.
        _ => {
            handle_unknown_command(command);
            EvalResult::Continue
        }
    }
}
