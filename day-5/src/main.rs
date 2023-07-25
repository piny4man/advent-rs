use itertools::Itertools;
use std::{fs, path::Path};

fn main() {
    let parsed_stacks =
        fs::read_to_string(Path::new("stacks.txt")).expect("Should contain crate stacks");
    let (boxes, rest) = parsed_stacks.split_once("\n\n").unwrap();
    let mut stacks = vec![vec![]; 9];

    for line in boxes.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..stacks.len() {
            let c = line[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                stacks[i].push(c as char);
            }
        }
    }
    let movements = rest
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|movement| movement.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .collect::<Vec<(usize, usize, usize)>>();
    print!("{:?}", move_boxes(&movements, stacks.clone()));
    print!("{:?}", move_multiple_boxes(&movements, stacks));
}

fn move_boxes(movements: &[(usize, usize, usize)], mut stacks: Vec<Vec<char>>) -> String {
    for &(times, from, to) in movements {
        for _ in 0..times {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}

fn move_multiple_boxes(movements: &[(usize, usize, usize)], mut stacks: Vec<Vec<char>>) -> String {
    for &(times, from, to) in movements {
        let boxes_len = stacks[to - 1].len() + times;
        stacks[to - 1].resize(boxes_len, ' ');
        for i in 0..times {
            let boxes = stacks[from - 1].pop().unwrap();
            stacks[to - 1][boxes_len - i - 1] = boxes;
        }
    }
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}
