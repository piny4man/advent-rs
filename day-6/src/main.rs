use itertools::Itertools;
use std::{fs, path::Path};

fn main() {
    let parsed_characters =
        fs::read_to_string(Path::new("characters.txt")).expect("Should contain characters");

    println!("{:?}", find_unique_characters(4, &parsed_characters));
    println!("{:?}", find_unique_characters(14, &parsed_characters));
}

fn find_unique_characters(size: usize, characters: &str) -> usize {
    size + characters
        .as_bytes()
        .windows(size)
        .position(|window| window.iter().tuple_combinations().all(|(a, b)| a != b))
        .unwrap()
}
