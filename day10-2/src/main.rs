use std::collections::HashSet;
use std::fs;
use Direction::E;
use Direction::N;
use Direction::S;
use Direction::W;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
    let pos_s = puzzle_input
        .lines()
        .enumerate()
        .map(|(id, line)| (id, line.chars().position(|x| x == 'S')))
        .filter(|(_, line_pos)| line_pos.is_some())
        .map(|(line_nr, line_pos)| (line_nr, line_pos.unwrap()))
        .next()
        .unwrap();
    let mut model: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|x| (x.to_owned()).chars().collect())
        .collect();
    let big_loop = compute_loop(pos_s, &mut model);

    assert!(big_loop.iter().all(|(y, x)| '.' != model[*y][*x]));

    let mut area = 0;
    for (y, row) in model.iter().enumerate() {
        let mut inside = false;
        let mut prev_corner = None;
        for (x, value) in row.iter().enumerate() {
            if big_loop.contains(&(y, x)) {
                match value {
                    '-' => continue,
                    '|' => inside = !inside,
                    'J' if prev_corner == Some('F') => {
                        prev_corner = None;
                        inside = !inside
                    }
                    '7' if prev_corner == Some('L') => {
                        prev_corner = None;
                        inside = !inside
                    }
                    corner => prev_corner = Some(*corner),
                }
            } else if inside {
                area += 1;
            }
        }
    }
    area
}

pub fn compute_loop(start: (usize, usize), model: &mut Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let (mut dir1, mut dir2) = compute_starting_directions(start.0, start.1, model);

    let mut big_loop = HashSet::new();
    big_loop.insert(start);
    big_loop.insert((dir1.1, dir1.2));
    big_loop.insert((dir2.1, dir2.2));

    loop {
        let mut tmp_next: Vec<(Direction, usize, usize)> = Vec::new();

        for (direction, y, x) in [dir1, dir2] {
            let (coming_from, (next_y, next_x)) = match compute_next_dir(direction, model[y][x]) {
                N => (S, (y - 1, x)),
                E => (W, (y, x + 1)),
                S => (N, (y + 1, x)),
                W => (E, (y, x - 1)),
            };

            if big_loop.contains(&(next_y, next_x)) {
                return big_loop;
            }
            big_loop.insert((next_y, next_x));
            tmp_next.push((coming_from, next_y, next_x));
        }
        dir1 = tmp_next[0];
        dir2 = tmp_next[1];
    }
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
    model: &mut Vec<Vec<char>>,
) -> ((Direction, usize, usize), (Direction, usize, usize)) {
    let mut next = Vec::new();
    if "|LJ".contains(model[start_y + 1][start_x]) {
        next.push((Direction::N, start_y + 1, start_x))
    }
    if start_y > 0 && "|7F".contains(model[start_y - 1][start_x]) {
        next.push((Direction::S, start_y - 1, start_x))
    }
    if start_x > 0 && "-LF".contains(model[start_y][start_x - 1]) {
        next.push((Direction::E, start_y, start_x - 1))
    }
    if "-J7".contains(model[start_y][start_x + 1]) {
        next.push((Direction::W, start_y, start_x + 1))
    }
    let dir1 = next[0];
    let dir2 = next[1];

    // replace starting position with the actual pipe it would be.
    model[start_y][start_x] = match (dir1.0, dir2.0) {
        (N, S) => '|',
        // we reach neighbors from north and east, hence the connector is '7'
        (N, E) => '7',
        // we reach neighbors from north and west, hence the connector is 'F'
        (N, W) => 'F',
        // ...
        (S, E) => 'J',
        // ...
        (S, W) => 'L',
        // ...
        (E, W) => '-',
        _ => panic!(),
    };
    (dir1, dir2)
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[cfg(test)]
mod test {
    use crate::solve_2;

    #[test]
    fn test1() {
        let example = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";

        assert_eq!(10, solve_2(example.to_owned()));
    }

    #[test]
    fn test2() {
        let example = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";

        assert_eq!(8, solve_2(example.to_owned()));
    }

    #[test]
    fn test3() {
        let example = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

        assert_eq!(4, solve_2(example.to_owned()));
    }
}
