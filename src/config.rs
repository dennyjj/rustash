use crate::r#enum::Command;
use std::error::Error;

pub struct Config {
    pub command: Command,
    pub msg: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            return Err("not enough arguments".into());
        }

        let command = match args[1].as_str() {
            "add" => Command::Add,
            "list" => Command::List,
            "show" => {
                if args.len() > 2 {
                    let index = args[2].parse::<usize>().map_err(|_| "invalid index")?;
                    Command::Show(Some(index))
                } else {
                    Command::Show(None)
                }
            }
            "clear" => Command::Clear,
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
