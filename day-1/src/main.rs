use std::{fs, path::Path};

fn main() {
    let parsed_calories = parse_calories();
    find_max_calories(parsed_calories)
}

fn parse_calories() -> String {
    fs::read_to_string(Path::new("calories_notes.txt")).expect("Should contain elves calories!")
}

fn find_max_calories(notes_calories: String) {
    let mut elve_calories: i32 = 0;
    let mut total_calories: Vec<i32> = Vec::new();

    for calories in notes_calories.lines() {
        if !calories.is_empty() {
            let calories_int: i32 = calories.parse().unwrap();
            elve_calories += calories_int; // total_calories.push(calorie);
        } else {
            total_calories.push(elve_calories);
            elve_calories = 0;
        }
    }

    let max_calories = total_calories.iter().max();
    match max_calories {
        Some(max) => println!("Max calories: {:?}", max),
        None => println!("No calories found!"),
    }
}
