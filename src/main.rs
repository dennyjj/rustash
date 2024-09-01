mod config;
mod utils;

use config::Config;
use dirs;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use utils::{create_file, get_json};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    match config.command.as_str() {
        "add" => {
            let file_path = get_json();
            let mut file = create_file(file_path);

            file.write_all(config.msg.unwrap().as_bytes())
                .expect("could not write to file");
        }
        "clear" => {
            let file_path = get_json();
            let mut file = create_file(file_path);

            file.write_all("".as_bytes())
                .expect("could not write to file");
        }
        _ => println!("Invalid command"),
    }
}
