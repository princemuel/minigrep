use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|exception| {
        println!("Problem parsing arguments: {exception}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let content = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!(" with text:\n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args.get(1).unwrap().to_string();
        let file_path = args.get(2).unwrap().to_string();

        Ok(Config { query, file_path })
    }
}
