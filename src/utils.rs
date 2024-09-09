use crate::config::Config;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;

pub fn get_json() -> PathBuf {
    let home_dir = dirs::home_dir().expect("could not find home directory");
    let mut file_path = home_dir;
    file_path.push(".rustash.json");
    file_path
}

pub fn add_note_to_file(file: &mut File, config: &Config) {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let new_contents = format!("{}\n{}", config.msg.as_ref().unwrap(), contents);
    file.set_len(0).expect("could not truncate file");
    file.write_all(new_contents.as_bytes())
        .expect("could not write to file");
}

pub fn list_notes(file: File) {
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

pub fn show_note_at_index(file: File, index: Option<usize>) {
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    if lines.is_empty() {
        println!("no notes yet...");
    } else if let Some(i) = index {
        if i < lines.len() {
            println!("{}) {}", i, lines[i]);
        } else {
            println!("note not found");
        }
    } else {
        println!("{}) {}", 0, lines[0]);
    }
}

pub fn remove_note_at_index(file: &mut File, index: usize) {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    if index < lines.len() {
        lines.remove(index);
        file.set_len(0).expect("could not truncate file");
        file.write_all(lines.join("\n").as_bytes())
            .expect("could not write to file");
        println!("note removed");
    } else {
        println!("note not found");
    }
}

pub fn clear_notes(file: &mut File) {
    file.set_len(0).expect("could not truncate file");
}
