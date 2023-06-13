use std::{fs, path::Path};

fn main() {
    let parsed_items = parse_items();
    let wrong_loaded_items = get_wrong_items_from_rucksacks(parsed_items);

    println!(
        "Sum of the priorities {:?}",
        wrong_loaded_items.iter().sum::<i32>()
    );
}

fn parse_items() -> String {
    fs::read_to_string(Path::new("rucksack-items.txt"))
        .expect("Should contain rucksacks items list")
}

fn get_wrong_items_from_rucksacks(rucksacks: String) -> Vec<i32> {
    let mut wrong_items: Vec<i32> = Vec::new();
    for rucksack in rucksacks.lines() {
        let total_rucksack_items = rucksack.len();
        let compartment_size = total_rucksack_items / 2;
        let rucksack_items = rucksack.chars().collect::<Vec<_>>();
        let left_compartment = rucksack_items[0..compartment_size].to_vec();
        let right_compartment = rucksack_items[compartment_size..].to_vec();
        let wrong_item = left_compartment.iter().find(|&item| {
            right_compartment
                .iter()
                .any(|right_item| item == right_item)
        });
        if let Some(item) = wrong_item {
            wrong_items.push(convert_item_to_number(&item.to_string()))
        }
    }
    wrong_items
}

fn convert_item_to_number(item: &str) -> i32 {
    match item {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        "j" => 10,
        "k" => 11,
        "l" => 12,
        "m" => 13,
        "n" => 14,
        "o" => 15,
        "p" => 16,
        "q" => 17,
        "r" => 18,
        "s" => 19,
        "t" => 20,
        "u" => 21,
        "v" => 22,
        "w" => 23,
        "x" => 24,
        "y" => 25,
        "z" => 26,
        "A" => 27,
        "B" => 28,
        "C" => 29,
        "D" => 30,
        "E" => 31,
        "F" => 32,
        "G" => 33,
        "H" => 34,
        "I" => 35,
        "J" => 36,
        "K" => 37,
        "L" => 38,
        "M" => 39,
        "N" => 40,
        "O" => 41,
        "P" => 42,
        "Q" => 43,
        "R" => 44,
        "S" => 45,
        "T" => 46,
        "U" => 47,
        "V" => 48,
        "W" => 49,
        "X" => 50,
        "Y" => 51,
        "Z" => 52,
        _ => 0,
    }
}
