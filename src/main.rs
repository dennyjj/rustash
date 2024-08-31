use dirs;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let input = &args[1];
        println!("Input argument: {}", input);

        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let mut file_path = PathBuf::from(home_dir);
        file_path.push(".rustash.json");

        let mut file = File::create(file_path).expect("Could not create file");

        file.write_all(input.as_bytes())
            .expect("Could not write to file");
    } else {
        println!("No input argument provided");
    }
}
