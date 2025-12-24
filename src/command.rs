use std::io;

pub enum BuiltInCommand {
    Type { _command: Box<Command> },
    Echo { args: Vec<String> },
    Exit,
}

pub enum Command {
    BuiltIn(BuiltInCommand),
    Executable { exe: String, args: Vec<String> },
    Empty,
}

fn parse_command(words: &[&str]) -> Command {
    if words.is_empty() {
        Command::Empty
    } else {
        match words[0] {
            "type" => {
                let inner_command = parse_command(&words[1..]);
                Command::BuiltIn(BuiltInCommand::Type {
                    _command: Box::new(inner_command),
                })
            }
            "echo" => {
                let args: Vec<String> = words[1..].iter().map(|s| s.to_string()).collect();
                Command::BuiltIn(BuiltInCommand::Echo { args })
            }
            "exit" => Command::BuiltIn(BuiltInCommand::Exit),
            _ => {
                // If it's not built-in, assume it's an executable.
                let exe = words[0].to_string();
                let args: Vec<String> = words[1..].iter().map(|s| s.to_string()).collect();
                Command::Executable { exe, args }
            }
        }
    }
}

pub fn read_command() -> Result<Command, io::Error> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    // This won't work well with quotes.
    let words: Vec<&str> = s.split_whitespace().collect();

    Ok(parse_command(&words))
}
