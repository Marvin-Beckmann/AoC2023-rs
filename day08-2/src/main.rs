use std::{collections::HashMap, fs};

use num::integer::lcm;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let mut lines = puzzle_input.lines();
    let directions = lines.next().unwrap();

    let map = fill_hash_map(puzzle_input.lines().skip(2));

    let starts: Vec<&&str> = map.keys().filter(|key| key.ends_with('A')).collect();

    // The LCM solution is not really general, but a general solution seemed to be very
    // time consuming..
    let rounds = starts
        .iter()
        .map(|start| follow_directions(start, directions, &map))
        .reduce(lcm)
        .unwrap();

    println!("{}\n", rounds)
}

pub fn follow_directions(
    start: &str,
    directions: &str,
    hash_map: &HashMap<&str, (&str, &str)>,
) -> usize {
    let mut steps = 0;
    let mut current = start;
    loop {
        for dir in directions.chars() {
            steps += 1;
            current = match dir {
                'L' => hash_map.get(current).unwrap().0,
                'R' => hash_map.get(current).unwrap().1,
                _ => panic!(),
            };
            println!("{}, {}", steps, current);
            if current.ends_with('Z') {
                return steps;
            }
        }
    }
}

pub fn fill_hash_map(s: std::iter::Skip<std::str::Lines<'_>>) -> HashMap<&str, (&str, &str)> {
    let mut map = HashMap::new();
    for line in s {
        let (start, left_right) = line.split_once(" = ").unwrap();
        let (left, right) = left_right
            .trim()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        map.insert(start, (left, right));
    }
    map
}
