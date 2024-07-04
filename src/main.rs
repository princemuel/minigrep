use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args);

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

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args.get(1).unwrap().to_string();
        let file_path = args.get(2).unwrap().to_string();

        Config { query, file_path }
    }
}
