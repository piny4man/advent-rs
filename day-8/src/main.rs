use itertools::Itertools;
use std::fs;

fn main() {
    let parsed_trees = fs::read_to_string("test-puzzle.txt").expect("Should have a grid of trees");
    let trees_grid: Vec<Vec<u32>> = parsed_trees
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let column_length = trees_grid.len();

    let hidden_trees = (1..column_length - 1)
        .cartesian_product(1..column_length - 1)
        .map(|(y, x)| {
            let tree_height = trees_grid[y][x];
            directions(&trees_grid, x, y)
                .iter()
                .map(|direction| direction.iter().all(|height| *height < tree_height))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count()
        + (column_length - 1) * 4;

    println!("{:?}", hidden_trees);
}

fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}
