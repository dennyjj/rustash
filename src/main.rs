mod config;
mod r#enum;
mod utils;

use config::Config;
use r#enum::Command;
use std::env;
use std::fs::OpenOptions;
use utils::{
    add_note_to_file, clear_notes, get_json, list_notes, remove_note_at_index, show_note_at_index,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let file_path = get_json();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .expect("could not open or create file");

    match config.command {
        Command::Add => add_note_to_file(&mut file, &config),
        Command::List => list_notes(file),
        Command::Show(index) => show_note_at_index(file, index),
        Command::Remove(index) => remove_note_at_index(&mut file, index),
        Command::Clear => clear_notes(&mut file),
    }
}
