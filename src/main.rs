use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (query, file_path) = parse_config(&args);

    print!("Searching for `{query}`");
    print!(" in file `{file_path}`");

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!(" with text:\n{content}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = args.get(1).unwrap();
    let file_path = args.get(2).unwrap();

    (query, file_path)
}
