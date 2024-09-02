mod config;
mod utils;

use config::{Command, Config};
use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use utils::{create_file, get_json, open_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    match config.command {
        Command::Add => {
            let file_path = get_json();
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .expect("could not open or create file");

            writeln!(file, "{}", config.msg.unwrap()).expect("could not write to file");
        }
        Command::List => {
            let file_path = get_json();
            let file = open_file(file_path);

            let reader = io::BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = line.expect("could not read line");
                println!("{}. {}", index, line);
            }
        }
        Command::Clear => {
            let file_path = get_json();
            let mut file = create_file(file_path);

            file.write_all("".as_bytes())
                .expect("could not write to file");
        }
    }
}
