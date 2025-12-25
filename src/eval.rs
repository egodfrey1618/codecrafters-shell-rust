use crate::command::{BuiltInCommand, Command};
use std::env::{split_paths, var_os};
use std::fs::metadata;
use std::os::linux::fs::MetadataExt;
use std::path::{MAIN_SEPARATOR, PathBuf};

pub enum EvalResult {
    /// Result of evaluating a command - this just tells the REPL whether
    /// to continue or exit.
    Continue,
    Exit,
}

fn path_is_executable_file(path: &PathBuf) -> bool {
    let metadata = {
        match metadata(path) {
            Err(_) => {
                // Most likely the path doesn't exist
                return false;
            }
            Ok(metadata) => metadata,
        }
    };

    // This checks if the file is executable by any user, which is
    // not quite the same as executable by the current process.
    // Curious to find out what Bash does here!
    // (It's also Linux-specific.)
    metadata.is_file() && (metadata.st_mode() & 0o111 != 0)
}

/// Given the first string of a command, try and parse it to a path on disk
/// This could either be relative or absolute.
fn locate_exe(exe: &String) -> Option<PathBuf> {
    let mut paths_unchecked: Vec<PathBuf> = vec![];
    if exe.contains(MAIN_SEPARATOR) {
        // This is something like ./prog. We want to treat it as a
        // relative path.
        let path = PathBuf::from(exe);
        paths_unchecked.push(path)
    } else {
        match var_os("PATH") {
            None => {
                panic!("PATH environment variable not set! Not much to do here, can't run stuff.");
            }
            Some(env_path) => {
                for dir in split_paths(&env_path) {
                    let path = dir.join(exe);
                    paths_unchecked.push(path)
                }
            }
        }
    };

    // Now we look through every path in paths_unchecked, picking
    // the first one that's an executable file.
    paths_unchecked.into_iter().find(path_is_executable_file)
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
            let exe_path = locate_exe(exe);
            match exe_path {
                None => println!("{exe}: not found"),
                Some(path) => println!("{} is {}", exe, path.display()),
            }
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
