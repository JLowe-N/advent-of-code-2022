use std::env;
use std::fs;
use std::process;

fn get_input(file_path: &str) {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have read file");
    println!("With text:\n{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Program parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Hello, world!");
    get_input(&config.file_path);
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Expected argument for file_path.");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
