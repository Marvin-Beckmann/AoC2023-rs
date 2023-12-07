use std::fs;

pub const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub(crate) fn main() {
    let mut puzzle_input = fs::read_to_string("src/puzzle_input.txt").unwrap();
    for (index, string) in DIGITS.iter().enumerate() {
        puzzle_input = puzzle_input.replace(string, &format!("{string}{}{string}", index + 1));
    }
    let mut sum = 0;
    for c in puzzle_input.split('\n') {
        let filtered: Vec<char> = c.chars().filter(|x| x.is_ascii_digit()).collect();
        let first = filtered.first().unwrap();
        let last = filtered.last().unwrap();
        let int: u32 = format!("{first}{last}").parse().unwrap();
        sum += int
    }
    print!("{}", sum);
}
