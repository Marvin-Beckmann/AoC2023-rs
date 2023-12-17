use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    let nr_row = puzzle_input.lines().count();
    let nr_cols = puzzle_input.lines().next().unwrap().len();
    let mut cols = Vec::new();
    for _ in 0..nr_cols {
        cols.push(Col {
            qube_shaped: Vec::new(),
            round_shaped: Vec::new(),
            nr_rows: nr_row,
        })
    }
    for (y, line) in puzzle_input.lines().enumerate() {
        for (x, elem) in line.chars().enumerate() {
            match elem {
                'O' => cols[x].round_shaped.push(y),
                '#' => cols[x].qube_shaped.push(y),
                _ => continue,
            }
        }
    }
    for col in &mut cols {
        col.shift_north();
    }
    cols.iter().map(|x| x.calculate_load()).sum()
}

#[derive(Debug)]
pub struct Col {
    qube_shaped: Vec<usize>,
    round_shaped: Vec<usize>,
    nr_rows: usize,
}

impl Col {
    pub fn calculate_load(&self) -> usize {
        self.round_shaped.iter().map(|id| self.nr_rows - id).sum()
    }

    pub fn shift_north(&mut self) {
        let mut blocked = self.qube_shaped.clone();
        let mut round: Vec<usize> = self.round_shaped.clone();
        round.sort();
        let mut new_rounds = Vec::new();
        for x in round {
            let filter = blocked.iter().filter(|&&id| id < x).map(|x| x + 1);
            let new_pos = filter.max().unwrap_or(0);
            blocked.push(new_pos);
            new_rounds.push(new_pos)
        }
        self.round_shaped = new_rounds
    }
}

#[cfg(test)]
mod test {
    use crate::solve_1;

    #[test]
    fn test() {
        let example = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

        assert_eq!(136, solve_1(example.to_owned()))
    }
}
