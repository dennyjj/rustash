mod config;
mod utils;

use config::{Command, Config};
use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Read, Write};
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
                .read(true)
                .write(true)
                .create(true)
                .open(file_path)
                .expect("could not open or create file");

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("could not read file");

            let new_contents = format!("{}\n{}", config.msg.unwrap(), contents);
            file.set_len(0).expect("could not truncate file");
            file.write_all(new_contents.as_bytes())
                .expect("could not write to file");
        }
        Command::List => {
            let file_path = get_json();
            let file = open_file(file_path);

            let reader = io::BufReader::new(file);
            let mut lines_found = false;
            for (index, line) in reader.lines().enumerate() {
                let line = line.expect("could not read line");
                println!("{}) {}", index, line);
                lines_found = true;
            }
            if !lines_found {
                println!("no notes yet...");
            }
        }
        Command::Show(index) => {
            let file_path = get_json();
            let file = open_file(file_path);

            let reader = io::BufReader::new(file);
            let lines: Vec<String> = reader
                .lines()
                .map(|line| line.expect("could not read line"))
                .collect();

            if let Some(i) = index {
                if i < lines.len() {
                    println!("{}) {}", i, lines[i]);
                } else {
                    println!("line not found");
                }
            } else {
                if !lines.is_empty() {
                    println!("{}) {}", 0, lines[0]);
                } else {
                    println!("no notes yet...");
                }
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
