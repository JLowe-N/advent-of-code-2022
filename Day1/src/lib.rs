use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = get_input(&config.file_path).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1)
    });
    let operation = &config.operation;
    let elves = parse_list_of_lists(input);

    let result;
    if operation == "max" {
        result = find_max_sum(elves);
    } else {
        result = find_sum_of_largest_n_sums(elves, &config.n_groups);
    }
    println!("{}", result);

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
    elves.push(food_calories.clone());
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

pub fn find_sum_of_largest_n_sums(two_dim_vec: Vec<Vec<i32>>, n_groups: &i32) -> i32 {
    let mut group_sums = Vec::new();
    println!("{}", two_dim_vec.len());
    for inner_list in two_dim_vec.iter() {
        let mut current_sum = 0;
        for number in inner_list.iter() {
            current_sum += number;
        }
        group_sums.push(current_sum);
    }
    if (group_sums.len() as i32) < *n_groups {
        return group_sums.iter().sum();
    }
    println!("{:?}", &group_sums[240..267]);
    group_sums.sort();
    let start_idx = (group_sums.len() as i32) - *n_groups;
    let mut sum_of_sums = 0;
    let mut current_idx = start_idx as usize;
    println!("{:?}", &group_sums[240..267]);
    while current_idx < (group_sums.len()) {
        sum_of_sums += group_sums[current_idx];
        current_idx += 1;
    }

    return sum_of_sums;
}

pub struct Config {
    file_path: String,
    operation: String,
    n_groups: i32
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Expected argument for file_path.");
        }
        let file_path = args[1].clone();
        let operation;
        if args.len() > 2 {
            operation = args[2].clone();
        } else {
            operation = "max".to_string();
        }
        let n_groups;
        if args.len() > 3 {
            n_groups = args[3].clone().parse::<i32>().unwrap();
        } else {
            n_groups = 0
        }

        Ok(Config { file_path, operation, n_groups })
    }
}
