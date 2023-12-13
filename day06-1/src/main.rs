use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let (time, distance) = puzzle_input.split_once('\n').unwrap();
    let time: Vec<usize> = time
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distance: Vec<usize> = distance
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let total_winning_options: usize = (0..time.len())
        .map(|race| compute_winning_options(time[race], distance[race]))
        .product();
    println!("{}", total_winning_options)
}

pub fn compute_winning_options(time: usize, distance: usize) -> usize {
    let mut count = 0;
    for charge_time in 1..time {
        // compute distance based on charged time
        let distance_after_charge = (time - charge_time) * charge_time;
        if distance_after_charge > distance {
            count += 1
        }
    }
    count
}
