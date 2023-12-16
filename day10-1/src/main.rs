use std::collections::HashSet;
use std::fs;
use Direction::E;
use Direction::N;
use Direction::S;
use Direction::W;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let pos_s = puzzle_input
        .lines()
        .enumerate()
        .map(|(id, line)| (id, line.chars().position(|x| x == 'S')))
        .filter(|(_, line_pos)| line_pos.is_some())
        .map(|(line_nr, line_pos)| (line_nr, line_pos.unwrap()))
        .next()
        .unwrap();
    let model: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|x| (x.to_owned()).chars().collect())
        .collect();
    let nr_rounds = follow_loop_compute_length(pos_s, model);
    println!("{}", nr_rounds)
}

pub fn follow_loop_compute_length(start: (usize, usize), model: Vec<Vec<char>>) -> usize {
    let mut next = compute_starting_directions(start.0, start.1, &model);

    let mut big_loop = HashSet::new();
    big_loop.insert(start);
    for (_, y, x) in next.clone() {
        big_loop.insert((y, x));
    }

    for round in 2.. {
        let dir1 = next[0];
        let dir2 = next[1];

        let mut tmp_next: Vec<(Direction, usize, usize)> = Vec::new();

        for (direction, y, x) in [dir1, dir2] {
            let (next_dir, (next_y, next_x)) = match compute_next_dir(direction, model[y][x]) {
                N => (S, (y - 1, x)),
                E => (W, (y, x + 1)),
                S => (N, (y + 1, x)),
                W => (E, (y, x - 1)),
            };

            if big_loop.contains(&(next_y, next_x)) {
                return round;
            }
            big_loop.insert((next_y, next_x));
            tmp_next.push((next_dir, next_y, next_x));
        }
        next = tmp_next;
    }
    unreachable!()
}

pub fn compute_next_dir(previous: Direction, current: char) -> Direction {
    match (previous, current) {
        (N, '|') => S,
        (N, 'L') => E,
        (N, 'J') => W,
        (S, '|') => N,
        (S, '7') => W,
        (S, 'F') => E,
        (E, '-') => W,
        (E, 'F') => S,
        (E, 'L') => N,
        (W, '-') => E,
        (W, 'J') => N,
        (W, '7') => S,
        _ => panic!(),
    }
}

pub fn compute_starting_directions(
    start_y: usize,
    start_x: usize,
    model: &Vec<Vec<char>>,
) -> Vec<(Direction, usize, usize)> {
    let mut next = Vec::new();
    if "|7F".contains(model[start_y - 1][start_x]) {
        next.push((Direction::S, start_y - 1, start_x))
    }
    if "|LJ".contains(model[start_y + 1][start_x]) {
        next.push((Direction::N, start_y + 1, start_x))
    }
    if "-LF".contains(model[start_y][start_x - 1]) {
        next.push((Direction::E, start_y, start_x - 1))
    }
    if "-J7".contains(model[start_y][start_x + 1]) {
        next.push((Direction::W, start_y, start_x + 1))
    }
    next
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}
