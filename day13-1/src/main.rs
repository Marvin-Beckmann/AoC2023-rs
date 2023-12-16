use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
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
        if (0..to_check).all(|j| lines[i - 1 - j] == lines[i + j]) {
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
        if (0..to_check).all(|j| {
            (0..lines.len()).all(|k| lines[k].chars().nth(i - 1 - j) == lines[k].chars().nth(i + j))
        }) {
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
        assert_eq!(5, search_vertical_symmetry(example1));

        let example2 =
            "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        assert_eq!(400, search_horizontal_symmetry(example2));
    }
}
