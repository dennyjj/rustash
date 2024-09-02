use dirs;
use std::fs::File;
use std::path::PathBuf;

pub fn get_json() -> PathBuf {
    let home_dir = dirs::home_dir().expect("could not find home directory");
    let mut file_path = PathBuf::from(home_dir);
    file_path.push(".rustash.json");
    return file_path;
}

pub fn open_file(file_path: PathBuf) -> File {
    return File::open(file_path).expect("could not open file");
}

pub fn create_file(file_path: PathBuf) -> File {
    return File::create(file_path).expect("could not create file");
}
