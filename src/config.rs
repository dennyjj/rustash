pub struct Config {
    pub command: String,
    pub msg: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();
        let msg = if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Config { command, msg })
    }
}
