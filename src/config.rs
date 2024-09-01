use std::fmt;

pub struct Config {
    pub command: String,
    pub msg: Option<String>,
}

enum Command {
    Add,
    Clear,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Add => write!(f, "add"),
            Command::Clear => write!(f, "clear"),
        }
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();
        if command == Command::Add.to_string() && args.len() < 3 {
            return Err("provide a message for add command");
        }

        let msg = if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Config { command, msg })
    }
}
