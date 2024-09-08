use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Add,
    List,
    Show(Option<usize>),
    Clear,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Add => write!(f, "add"),
            Command::List => write!(f, "list"),
            Command::Show(Some(index)) => write!(f, "show {}", index),
            Command::Show(None) => write!(f, "show"),
            Command::Clear => write!(f, "clear"),
        }
    }
}
