use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let content = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!(" with text:\n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args.get(1).unwrap();
    let file_path = args.get(2).unwrap();

    Config { query: query.to_string(), file_path: file_path.to_string() }
}
