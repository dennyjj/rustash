use dirs;
use std::path::PathBuf;

pub fn get_json() -> PathBuf {
    let home_dir = dirs::home_dir().expect("could not find home directory");
    let mut file_path = PathBuf::from(home_dir);
    file_path.push(".rustash.json");
    return file_path;
}
