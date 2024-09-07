use config::Config;
use r#enum::Command;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use utils::get_json;

mod config;
mod r#enum;
mod utils;

fn add_note_to_file(file: &mut File, config: &Config) {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let new_contents = format!("{}\n{}", config.msg.as_ref().unwrap(), contents);
    file.set_len(0).expect("could not truncate file");
    file.write_all(new_contents.as_bytes())
        .expect("could not write to file");
}

fn list_notes(file: File) {
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

fn show_note_at_index(file: File, index: Option<usize>) {
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    if let Some(i) = index {
        if i < lines.len() {
            println!("{}) {}", i, lines[i]);
        } else {
            println!("note not found");
        }
    } else {
        if !lines.is_empty() {
            println!("{}) {}", 0, lines[0]);
        } else {
            println!("no notes yet...");
        }
    }
}

fn clear_notes(file: &mut File) {
    file.set_len(0).expect("could not truncate file");
}

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
        Command::Clear => clear_notes(&mut file),
    }
}
