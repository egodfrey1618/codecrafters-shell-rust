use std::io;

pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

pub fn read_command() -> Result<Command, io::Error> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    // This won't work well with quotes.
    let words: Vec<String> = s.trim().split(" ").map(|s| s.to_string()).collect();

    let command: String;
    let args: Vec<String>;

    if words.is_empty() {
        // If nothing is passed, treat this as an empty command.
        command = String::new();
        args = vec![];
    } else {
        command = words[0].clone();
        args = words[1..].iter().map(|s| s.to_string()).collect();
    }
    Ok(Command { command, args })
}
