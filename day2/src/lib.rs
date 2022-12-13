use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = get_input(&config.file_path).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1)
    });
    score_rounds_part2(input);

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

pub fn score_rounds_part2(rounds: String) -> () {
    let mut player_abc_score = 0;
    let mut player_xyz_score = 0;
    const WIN_VALUE: u32 = 6;
    const TIE_VALUE: u32 = 3;
    const LOSS_VALUE: u32 = 0;
    let play_value: HashMap<&str, u32> = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let xyz_wins = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);
    let xyz_loses = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);
    let xyz_play_result = HashMap::from([("X", "lose"), ("Y", "tie"), ("Z", "win")]);
    for round in rounds.lines() {
        let plays: Vec<&str> = round.split(" ").collect();
        player_abc_score += play_value[plays[0]];
        let xyz_play_translation;
        if xyz_play_result[plays[1]] == "tie" {
            player_abc_score += TIE_VALUE;
            player_xyz_score += TIE_VALUE;
            xyz_play_translation = plays[0];
        } else if xyz_play_result[plays[1]] == "lose" {
            player_abc_score += WIN_VALUE;
            player_xyz_score += LOSS_VALUE;
            xyz_play_translation = xyz_loses[plays[0]];
        } else {
            player_abc_score += LOSS_VALUE;
            player_xyz_score += WIN_VALUE;
            xyz_play_translation = xyz_wins[plays[0]];
        }
        println!("{}", xyz_play_translation);
        player_xyz_score += play_value[xyz_play_translation];
    }
    println!(
        "Player ABC {} : Player XYZ {}",
        player_abc_score, player_xyz_score
    )
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
