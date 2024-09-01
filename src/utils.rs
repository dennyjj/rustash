pub fn get_json() -> PathBuf {
    let home_dir = dirs::home_dir().expect("could not find home directory");
    let mut file_path = PathBuf::from(home_dir);
    file_path.push(".rustash.json");
    return file_path;
}

pub fn create_file(file_path: PathBuf) -> File {
    return File::create(file_path).expect("could not create file");
}
