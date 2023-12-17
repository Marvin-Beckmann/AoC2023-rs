use std::{fs, os::unix::ffi::OsStrExt, str::from_utf8};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    puzzle_input.split(',').map(compute_hash).sum()
}

pub fn compute_hash(str: &str) -> usize {
    str.chars()
        .fold(0, |acc, char| 17 * (acc + (char as usize)) % 256)
}

#[cfg(test)]
mod test {
    use crate::solve_1;

    #[test]
    fn test() {
        let example = "rn=1";

        assert_eq!(30, solve_1(example.to_owned()))
    }
}
