use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = get_input(&config.file_path).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1)
    });
    let rounds = score_rounds(input);

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

pub fn score_rounds(rounds: String) -> () {
    let mut player_abc_score = 0;
    let mut player_xyz_score = 0;
    const WIN_VALUE: u32 = 6;
    const TIE_VALUE: u32 = 3;
    const LOSS_VALUE: u32 = 0;
    let play_value: HashMap<&str, u32> =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let winning_plays: HashMap<&str, &str> = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);
    let tie_plays: HashMap<&str, &str> = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    for round in rounds.lines() {
        let plays: Vec<&str> = round.split(" ").collect();
        player_abc_score += play_value[plays[0]];
        player_xyz_score += play_value[plays[1]];
        if tie_plays[plays[0]] == plays[1] {
            player_abc_score += TIE_VALUE;
            player_xyz_score += TIE_VALUE;
        } else if winning_plays[plays[0]] == plays[1] {
            player_abc_score += WIN_VALUE;
            player_xyz_score += LOSS_VALUE;
        } else {
            player_abc_score += LOSS_VALUE;
            player_xyz_score += WIN_VALUE;
        }
    }
    println!(
        "Player ABC {} : Player XYZ {}",
        player_abc_score, player_xyz_score
    )
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
    return max_sum;
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
