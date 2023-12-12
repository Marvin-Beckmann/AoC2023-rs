use itertools::Itertools;
use std::{fmt::Error, fs, str::FromStr};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let mut puzzle_iter = puzzle_input.split("\n\n");
    let seeds_and_range: Vec<usize> = puzzle_iter
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let maps_vec: Vec<Vec<Map>> = puzzle_iter
        .map(|map_str| {
            map_str
                .lines()
                .skip(1)
                .map(|line| Map::from_str(line).unwrap())
                .collect()
        })
        .collect();
    println!("{}", map_seeds_and_range(seeds_and_range, maps_vec))
}

pub fn map_seeds_and_range(seeds_and_range: Vec<usize>, maps_vec: Vec<Vec<Map>>) -> usize {
    seeds_and_range
        .iter()
        .tuples()
        .flat_map(|(start, len)| fully_map_range((*start, start + len - 1), &maps_vec))
        .map(|(x, _)| x)
        .min()
        .unwrap()
}

pub fn fully_map_range(
    (start, end): (usize, usize),
    maps_vec: &Vec<Vec<Map>>,
) -> Vec<(usize, usize)> {
    let mut ranges = vec![(start, end)];
    for maps in maps_vec {
        ranges = ranges
            .iter()
            .flat_map(|range| map_range(*range, maps))
            .collect()
    }
    ranges
}

pub fn map_range((start, end): (usize, usize), maps: &Vec<Map>) -> Vec<(usize, usize)> {
    let mut mapped = Vec::new();
    let mut unmapped = vec![(start, end)];
    for map in maps {
        let (map_start, map_end) = (map.source_start, (map.source_start + map.range_length));
        let mut tmp_unmapped = Vec::new();
        for (seed_start, seed_end) in unmapped {
            let left = (seed_start, seed_end.min(map_start));
            let right = (seed_start.max(map_end), seed_end);
            let middle = (seed_start.max(map_start), seed_end.min(map_end));

            if middle.0 < middle.1 {
                mapped.push((
                    middle.0 - map_start + map.dest_start,
                    middle.1 - map_start + map.dest_start,
                ))
            }
            if left.0 < left.1 {
                tmp_unmapped.push(left)
            }
            if right.0 < right.1 {
                tmp_unmapped.push(right)
            }
        }
        unmapped = tmp_unmapped
    }
    mapped.extend(unmapped);
    mapped
}

pub struct Map {
    source_start: usize,
    dest_start: usize,
    range_length: usize,
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
