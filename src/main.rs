use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let query = args.get(1).unwrap();
    let file_path = args.get(2).unwrap();

    print!("Searching for {query} ");
    print!("in file {file_path}");
}
