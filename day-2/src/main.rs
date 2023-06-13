use std::{fs, path::Path, str::FromStr};

enum Movements {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Movements {
    type Err = &'static str;
    fn from_str(movement: &str) -> Result<Self, Self::Err> {
        match movement.to_lowercase().as_ref() {
            "a" | "x" => Ok(Movements::Rock),
            "b" | "y" => Ok(Movements::Paper),
            "c" | "z" => Ok(Movements::Scissors),
            _ => Err("Not a valid movement"),
        }
    }
}

impl From<Movements> for i32 {
    fn from(movement: Movements) -> i32 {
        match movement {
            Movements::Rock => 1,
            Movements::Paper => 2,
            Movements::Scissors => 3,
        }
    }
}

enum TurnResult {
    Win,
    Draw,
    Lose,
}

impl From<TurnResult> for i32 {
    fn from(result: TurnResult) -> i32 {
        match result {
            TurnResult::Win => 6,
            TurnResult::Draw => 3,
            TurnResult::Lose => 0,
        }
    }
}

fn main() {
    let parsed_turns = parse_turns();
    play_turns(parsed_turns)
}

fn parse_turns() -> String {
    fs::read_to_string(Path::new("turns.txt")).expect("Should contain turns!")
}

fn play_turns(turns: String) {
    let mut player1_score: i32 = 0;
    let mut player2_score: i32 = 0;

    for turn in turns.lines() {
        if !turn.is_empty() {
            let turn_movements: Vec<&str> = turn.split_whitespace().collect();
            let player1_movement = Movements::from_str(turn_movements[0]).unwrap();
            let player2_movement = Movements::from_str(turn_movements[1]).unwrap();
            let player1_result = compare_movements(&player1_movement, &player2_movement);
            let player2_result = compare_movements(&player2_movement, &player1_movement);
            player1_score += i32::from(player1_result) + i32::from(player1_movement);
            player2_score += i32::from(player2_result) + i32::from(player2_movement);
        }
    }

    println!("Player 1 score: {:?}", player1_score);
    println!("Player 2 score: {:?}", player2_score);
}

fn compare_movements(player1_movement: &Movements, player2_movement: &Movements) -> TurnResult {
    match (player1_movement, player2_movement) {
        (Movements::Rock, Movements::Rock) => TurnResult::Draw,
        (Movements::Rock, Movements::Paper) => TurnResult::Lose,
        (Movements::Rock, Movements::Scissors) => TurnResult::Win,
        (Movements::Paper, Movements::Rock) => TurnResult::Win,
        (Movements::Paper, Movements::Paper) => TurnResult::Draw,
        (Movements::Paper, Movements::Scissors) => TurnResult::Lose,
        (Movements::Scissors, Movements::Rock) => TurnResult::Lose,
        (Movements::Scissors, Movements::Paper) => TurnResult::Win,
        (Movements::Scissors, Movements::Scissors) => TurnResult::Draw,
    }
}
