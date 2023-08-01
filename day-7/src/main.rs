use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let parsed_terminal_history =
        fs::read_to_string(Path::new("history.txt")).expect("Should contain terminal history");
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in parsed_terminal_history.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => affected.push(name),
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }
    let files_under_100_000: u32 = sizes
        .clone()
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum();
    println!("Files under 100000 size: {:?}", files_under_100_000);

    let disk = 70_000_000;
    let unused_space_needed = 30_000_000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available_space = disk - root;
    let smallest_space_to_free = sizes
        .into_values()
        .filter(|size| available_space + size >= unused_space_needed)
        .min()
        .unwrap();
    println!(
        "Smallest file to delete for the update: {:?}",
        smallest_space_to_free
    );
}
