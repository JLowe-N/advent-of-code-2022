use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let input = get_input(&config.file_path).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1)
    });
    let elves = parse_list_of_lists(input);
    let max_sum = find_max_sum(elves);
    println!("{}", max_sum);

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

    #[test]
    fn rev_max() {
        let input = vec![vec![3000, 2000, 1000], vec![5000, 7000]];

        assert_eq!(12000, find_max_sum(input))
    }
}

pub fn parse_list_of_lists(string: String) -> Vec<Vec<i32>> {
    let mut elves = Vec::new();
    let mut food_calories = Vec::new();
    for line in string.lines() {
        if line.is_empty() && food_calories.len() > 0 {
            elves.push(food_calories.clone());
            food_calories = Vec::new();
        } else {
            let calories = line.parse::<i32>().unwrap();
            food_calories.push(calories);
        }
    }
    return elves
}

pub fn find_max_sum(input: Vec<Vec<i32>>) -> i32 {
    let mut max_sum = 0;
    for elf_foods in input.iter() {
        let mut current_sum = 0;
        for food_calories in elf_foods.iter() {
            current_sum += food_calories;
        }
        if current_sum > max_sum {
            max_sum = current_sum
        }
    }
    return max_sum
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
