use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let input = &args[1];
        println!("Input argument: {}", input);
    } else {
        println!("No input argument provided");
    }
}
