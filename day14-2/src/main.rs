use std::{collections::HashMap, fmt::Error, fs, str::FromStr};

pub const MAX_ITER: usize = 1000000000;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
    let mut board = Board::from_str(&puzzle_input).unwrap();

    let mut hash_map = HashMap::new();
    board.shift_north();
    hash_map.insert(board.compute_string(), 1);

    for i in 2..=(4 * MAX_ITER) {
        board.rotate_90_right();
        board.shift_north();
        let board_string = board.compute_string();
        if let Some(prev) = hash_map.get(&board_string) {
            let actual_rotations_left = (4 * MAX_ITER - i) % (i - prev);
            for _ in 0..actual_rotations_left {
                board.rotate_90_right();
                board.shift_north();
            }
            break;
        }
        hash_map.insert(board.compute_string(), i);
    }
    board.rotate_90_right();

    board.calculate_load()
}

#[derive(Debug)]
pub struct Board {
    qube_shaped: Vec<(usize, usize)>,
    round_shaped: Vec<(usize, usize)>,
    size: usize,
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s.lines().count();
        let mut board = Board {
            qube_shaped: Vec::new(),
            round_shaped: Vec::new(),
            size,
        };
        for (y, line) in s.lines().enumerate() {
            for (x, elem) in line.chars().enumerate() {
                match elem {
                    'O' => board.round_shaped.push((y, x)),
                    '#' => board.qube_shaped.push((y, x)),
                    _ => continue,
                }
            }
        }
        Ok(board)
    }
}

impl Board {
    pub fn calculate_load(&self) -> usize {
        self.round_shaped.iter().map(|(y, _)| self.size - y).sum()
    }

    pub fn shift_north(&mut self) {
        let mut blocked = self.qube_shaped.clone();
        self.round_shaped.sort_by_key(|(y, _)| *y);
        let mut new_rounds = Vec::new();
        for (y, x) in &self.round_shaped {
            let filter = blocked
                .iter()
                .filter(|(b_y, b_x)| b_x == x && b_y < y)
                .map(|(b_y, _)| (b_y + 1, *x));
            let new_pos = filter.max_by_key(|(y, _)| *y).unwrap_or((0, *x));
            blocked.push(new_pos);
            new_rounds.push(new_pos)
        }
        self.round_shaped = new_rounds
    }

    pub fn rotate_90_right(&mut self) {
        self.qube_shaped = self
            .qube_shaped
            .iter()
            .map(|(y, x)| (*x, self.size - 1 - y))
            .collect();
        self.round_shaped = self
            .round_shaped
            .iter()
            .map(|(y, x)| (*x, self.size - 1 - y))
            .collect();
    }

    pub fn compute_string(&self) -> String {
        let mut rounds = self.round_shaped.clone();
        rounds.sort();
        let mut qubes = self.qube_shaped.clone();
        qubes.sort();
        format!(
            "{};{}",
            rounds
                .iter()
                .map(|(y, x)| format!("({},{})", y, x))
                .collect::<Vec<String>>()
                .join(""),
            qubes
                .iter()
                .map(|(y, x)| format!("({},{})", y, x))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{solve_2, Board};
    use std::str::FromStr;

    #[test]
    fn test() {
        let example = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

        let mut board = Board::from_str(example).unwrap();
        board.shift_north();
        assert_eq!(136, board.calculate_load());

        assert_eq!(64, solve_2(example.to_owned()))
    }
}
