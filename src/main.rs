use std::{env, process};

use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|exception| {
        eprintln!("Problem parsing arguments: {exception}");
        process::exit(1);
    });

    println!(
        "Searching for the keyword `{}` in file `{}`",
        config.query, config.file_path
    );

    if let Err(exception) = minigrep::run(config) {
        eprintln!("Application Error: {exception}");
        process::exit(1);
    };
}
