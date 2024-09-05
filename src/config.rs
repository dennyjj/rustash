use std::error::Error;
use std::fmt;

pub struct Config {
    pub command: Command,
    pub msg: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add,
    List,
    Clear,
    Show(Option<usize>),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Add => write!(f, "add"),
            Command::List => write!(f, "list"),
            Command::Clear => write!(f, "clear"),
            Command::Show(Some(index)) => write!(f, "show {}", index),
            Command::Show(None) => write!(f, "show"),
        }
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            return Err("not enough arguments".into());
        }

        let command = match args[1].as_str() {
            "add" => Command::Add,
            "list" => Command::List,
            "clear" => Command::Clear,
            "show" => {
                if args.len() > 2 {
                    let index = args[2].parse::<usize>().map_err(|_| "invalid index")?;
                    Command::Show(Some(index))
                } else {
                    Command::Show(None)
                }
            }
            _ => return Err("invalid command, accepted commands: add, list, clear, show".into()),
        };

        if let Command::Add = command {
            if args.len() < 3 {
                return Err("provide a message for add command".into());
            }
        }

        let msg = if args.len() > 2 && command != Command::Show(None) {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Config { command, msg })
    }
}
