use crate::r#enum::Command;
use std::error::Error;

pub struct Config {
    pub command: Command,
    pub msg: Option<String>,
}

fn get_command(args: &[String]) -> Result<Command, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("not enough arguments".into());
    }

    match args[1].as_str() {
        "add" => Ok(Command::Add),
        "list" => Ok(Command::List),
        "show" => {
            if args.len() > 2 {
                let index = args[2].parse::<usize>().map_err(|_| "invalid index")?;
                Ok(Command::Show(Some(index)))
            } else {
                Ok(Command::Show(None))
            }
        }
        "remove" => {
            if args.len() > 2 {
                let index = args[2].parse::<usize>().map_err(|_| "invalid index")?;
                Ok(Command::Remove(index))
            } else {
                Err("provide an index for remove command".into())
            }
        }
        "clear" => Ok(Command::Clear),
        _ => Err("invalid command, accepted commands: add, list, clear, show".into()),
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            return Err("not enough arguments".into());
        }
        let command = get_command(args);

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
