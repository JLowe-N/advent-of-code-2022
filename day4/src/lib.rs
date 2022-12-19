use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = get_input(&config.file_path).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1)
    });
    part_one_count_overlaps(&input);
    part_two_count_overlaps(&input);

    Ok(())
}

pub fn get_input(file_path: &str) -> Result<String, Box<dyn Error>> {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
}

pub fn part_one_count_overlaps(input: &str) -> () {
    let mut overlap_count = 0;
    for line in input.lines() {
        if is_overlap(&line) {
            overlap_count += 1;
        }
    }
    println!("{}", overlap_count);
}

pub fn part_two_count_overlaps(input: &str) -> () {
    let mut overlap_count = 0;
    for line in input.lines() {
        if is_partial_overlap(&line) {
            overlap_count += 1;
        }
    }
    println!("{}", overlap_count);
}

pub fn is_overlap(range_pair: &str) -> bool {
    let mut is_overlapping = false;
    let pairs: Vec<&str> = range_pair.split(',').collect();
    let left_range: Vec<i32> = pairs[0]
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let right_range: Vec<i32> = pairs[1]
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    if left_range[0] <= right_range[0] && left_range[1] >= right_range[1] {
        is_overlapping = true;
    } else if right_range[0] <= left_range[0] && right_range[1] >= left_range[1] {
        is_overlapping = true;
    }
    is_overlapping
}

pub fn is_partial_overlap(range_pair: &str) -> bool {
    let mut is_overlapping = true;
    let pairs: Vec<&str> = range_pair.split(',').collect();
    let left_range: Vec<i32> = pairs[0]
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let right_range: Vec<i32> = pairs[1]
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    if left_range[1] < right_range[0] || right_range[1] < left_range[0] {
        is_overlapping = false;
    }
    is_overlapping
}

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Expected argument for file_path.");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
