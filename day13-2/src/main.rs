use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
    let mut current = 0;
    for block in puzzle_input.split("\n\n") {
        current += search_horizontal_symmetry(block);
        current += search_vertical_symmetry(block);
    }
    current
}

pub fn search_horizontal_symmetry(puzzle_input: &str) -> usize {
    let lines: Vec<&str> = puzzle_input.lines().collect();
    let n = lines.len();
    let mut current = 0;
    for i in 1..n {
        let to_check = i.min(n - i);
        let mut err_count = 0;
        for j in 0..to_check {
            for k in 0..lines[0].len() {
                if lines[i - 1 - j].chars().nth(k) != lines[i + j].chars().nth(k) {
                    err_count += 1
                }
            }
        }
        if err_count == 1 {
            current += 100 * i;
        }
    }
    current
}

pub fn search_vertical_symmetry(puzzle_input: &str) -> usize {
    let lines: Vec<&str> = puzzle_input.lines().collect();
    let n = lines[0].len();
    let mut current = 0;
    for i in 1..n {
        let to_check = i.min(n - i);
        let mut err_count = 0;
        for j in 0..to_check {
            for k in 0..lines.len() {
                if lines[k].chars().nth(i - 1 - j) != lines[k].chars().nth(i + j) {
                    err_count += 1
                }
            }
        }
        if err_count == 1 {
            current += i;
        }
    }
    current
}

#[cfg(test)]
mod test {
    use crate::{search_horizontal_symmetry, search_vertical_symmetry};

    #[test]
    fn test() {
        let example1 =
            "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        assert_eq!(300, search_horizontal_symmetry(example1));

        let example2 =
            "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        assert_eq!(100, search_horizontal_symmetry(example2));
    }
}
