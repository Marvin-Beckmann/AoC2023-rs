use std::{fmt::Error, fs, str::FromStr};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let mut puzzle_iter = puzzle_input.split("\n\n");
    let seeds: Vec<usize> = puzzle_iter
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut current_map = seeds.clone();
    for x in puzzle_iter {
        let maps = x
            .lines()
            .skip(1)
            .map(|line| Map::from_str(line).unwrap())
            .collect();
        current_map = current_map.iter().map(|x| map_value(*x, &maps)).collect();
    }
    let smallest_location = current_map.iter().min().unwrap();
    println!("{smallest_location}")
}

pub struct Map {
    source_start: usize,
    dest_start: usize,
    range_length: usize,
}

impl Map {
    pub fn map(&self, value: usize) -> Option<usize> {
        if self.source_start <= value && value < self.range_length + self.source_start {
            Some(self.dest_start + value - self.source_start)
        } else {
            None
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace().map(|x| x.parse().unwrap());
        Ok(Self {
            dest_start: split.next().unwrap(),
            source_start: split.next().unwrap(),
            range_length: split.next().unwrap(),
        })
    }
}

pub fn map_value(value: usize, maps: &Vec<Map>) -> usize {
    let mut new_val = maps
        .iter()
        .map(|map| map.map(value))
        .filter(|x| x.is_some());
    new_val.next().unwrap_or(Some(value)).unwrap()
}
