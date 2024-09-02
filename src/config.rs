use std::fmt;

pub struct Config {
    pub command: Command,
    pub msg: Option<String>,
}

#[derive(Debug)]
pub enum Command {
    Add,
    List,
    Clear,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Add => write!(f, "add"),
            Command::List => write!(f, "list"),
            Command::Clear => write!(f, "clear"),
        }
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let command = match args[1].as_str() {
            "add" => Command::Add,
            "list" => Command::List,
            "clear" => Command::Clear,
            _ => return Err("invalid command, accepted commands: add, list, clear"),
        };

        if let Command::Add = command {
            if args.len() < 3 {
                return Err("provide a message for add command");
            }
        }

        let msg = if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Config { command, msg })
    }
}
