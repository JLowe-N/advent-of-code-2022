use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = get_input(&config.file_path).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1)
    });
    let rucksack_value = part_one_calculate_duplicate_value(input.clone());
    println!("{}", rucksack_value);
    let badge_values = part_two_calculate_badge_values(input.clone());
    println!("{}", badge_values);

    Ok(())
}

pub fn part_two_calculate_badge_values(input: String) -> u32 {
    let mut badge_values = 0;
    let mut rucksack_group = vec![];
    for rucksack in input.lines() {
        rucksack_group.push(rucksack);
        if rucksack_group.len() == 3 {
            let character = find_common_item(rucksack_group.clone()).unwrap_or_else(|err| {
                process::exit(1);
            });
            badge_values += get_priority(character);
            rucksack_group.clear();
        }
    }
    badge_values
}

pub fn part_one_calculate_duplicate_value(input: String) -> u32 {
    let mut rucksack_value = 0;
    for line in input.lines() {
        let duplicate_letter = get_rucksack_duplicate(line).unwrap();
        rucksack_value += get_priority(duplicate_letter);
    }
    rucksack_value
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

/// Finds the duplicate item a-zA-Z in the 2 compartments
/// of the rucksack string
///
/// # Arguments
///
/// * `rucksack` - A string that when evenly split describes two
/// compartments filled with items a-zA-Z with priorities 1-52 according
/// to ANSI order
pub fn get_rucksack_duplicate(rucksack: &str) -> Result<char, String> {
    let rucksack_size = rucksack.len();
    let compartments = rucksack.split_at(rucksack_size / 2);
    let mut visited = HashMap::new();
    // Look into graphemes, vector u8, or bytes here
    for item in compartments.0.chars() {
        visited.insert(item, true);
    }
    for item in compartments.1.chars() {
        if visited.contains_key(&item) {
            return Ok(item);
        }
    }
    Err("No duplicate items found in rucksack compartments".to_string())
}

pub fn get_priority(letter: char) -> u32 {
    let ALPHABET = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut priority = HashMap::new();
    for (i, x) in ALPHABET.chars().enumerate() {
        priority.insert(x, i as u32 + 1);
    }
    priority
        .get(&letter)
        .expect("Letter priority not found")
        .clone()
}

pub fn find_common_item(rucksacks: Vec<&str>) -> Result<char, String> {
    let target_count = rucksacks.len();
    let mut counts: HashMap<char, usize> = HashMap::new();
    for sack in rucksacks.iter() {
        // don't count duplicates within the same sack
        let mut visited: HashMap<char, bool> = HashMap::new();
        for character in sack.chars() {
            if visited.contains_key(&character) {
                continue
            }
            visited.insert(character, true);

            if !counts.contains_key(&character) {
                counts.insert(character, 0);
            }
            let current_count = counts.get(&character).unwrap();
            let next_count = current_count + 1;
            if next_count == target_count {
                println!("{}", character);
                return Ok(character);
            }
            counts.insert(character, current_count + 1);
        }
    }
    return Err("Group of rucksacks had no common item.".to_string());
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
