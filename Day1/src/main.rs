use std::env;
use std::fs;

fn get_input(file_path: &str) {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have read file");
    println!("With text:\n{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Hello, world!");
    get_input(&config.file_path);
}

struct Config {
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let file_path = args[1].clone();

        Config { file_path }
    }
}
