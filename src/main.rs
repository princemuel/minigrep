use std::error::Error;
use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|exception| {
        println!("Problem parsing arguments: {exception}");
        process::exit(1);
    });

    println!(
        "Searching for the keyword `{}` in file `{}`",
        config.query, config.file_path
    );

    if let Err(exception) = run(config) {
        println!("Application Error: {exception}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    println!(" with text:\n{content}");
    Ok(())
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
