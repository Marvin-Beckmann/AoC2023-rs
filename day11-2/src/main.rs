use itertools::Itertools;
use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for (y, line) in puzzle_input.lines().enumerate() {
        for (x, entry) in line.chars().enumerate() {
            if entry == '#' {
                indices.push((y, x));
            }
        }
    }

    let empty_rows: Vec<bool> = puzzle_input
        .lines()
        .map(|line| line.chars().all(|char| char == '.'))
        .collect();

    let row_length = puzzle_input.lines().next().unwrap().len();
    let empty_columns: Vec<bool> = (0..row_length)
        .map(|x| {
            puzzle_input
                .lines()
                .all(|line| line.chars().nth(x) == Some('.'))
        })
        .collect();
    indices
        .iter()
        .tuple_combinations()
        .map(|(first, second)| distance_galaxies(first, second, &empty_rows, &empty_columns))
        .sum()
}

pub fn distance_galaxies(
    first: &(usize, usize),
    second: &(usize, usize),
    empty_rows: &Vec<bool>,
    empty_columns: &Vec<bool>,
) -> usize {
    let mut count = 0;
    let min_y = first.0.min(second.0);
    let max_y = first.0.max(second.0);
    for y in min_y..max_y {
        count += match empty_rows[y] {
            true => 1000000,
            false => 1,
        }
    }

    let min_x = first.1.min(second.1);
    let max_x = first.1.max(second.1);
    for x in min_x..max_x {
        count += match empty_columns[x] {
            true => 1000000,
            false => 1,
        }
    }
    count
}
