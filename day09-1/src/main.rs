use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let res: i64 = puzzle_input
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .map(predict_next)
        .sum();
    println!("{}", res)
}

pub fn predict_next(numbers: Vec<i64>) -> i64 {
    if numbers.iter().all(|x| x == &0) {
        0
    } else {
        let new_numbers: Vec<i64> = (0..(numbers.len() - 1))
            .map(|x| numbers[x + 1] - numbers[x])
            .collect();
        numbers[numbers.len() - 1] + predict_next(new_numbers)
    }
}
