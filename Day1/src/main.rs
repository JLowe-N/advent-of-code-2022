use std::env;
use std::fs;

fn get_input(file_path: &str) {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have read file");
    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> &str {
    let file_path = &args[1];

    return file_path;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_config(&args);
    println!("Hello, world!");
    get_input(&file_path);
}
