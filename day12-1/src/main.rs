use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    puzzle_input.lines().map(compute_possible_solutions).sum()
}

pub fn compute_possible_solutions(line: &str) -> usize {
    let (spring_records, check_sums) = line.split_once(' ').unwrap();

    if let Some(position) = spring_records.chars().position(|x| x == '?') {
        let before = &line[..position];
        let after = &line[(position + 1)..];
        return compute_possible_solutions(&format!("{}#{}", before, after))
            + compute_possible_solutions(&format!("{}.{}", before, after));
    }

    let mut sums: Vec<usize> = check_sums.split(',').map(|x| x.parse().unwrap()).collect();

    let mut sum_id = 0;
    let mut start_sequence = false;
    for char in spring_records.chars() {
        if char == '#' {
            start_sequence = true;
            if sum_id > sums.len() - 1 || sums[sum_id] == 0 {
                return 0;
            }
            sums[sum_id] -= 1;
        } else if char == '.' && start_sequence {
            start_sequence = false;
            if sums[sum_id] != 0 {
                return 0;
            }
            sum_id += 1;
        }
    }
    if sums.iter().all(|x| *x == 0) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::solve_1;

    #[test]
    fn test1() {
        let example = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

        assert_eq!(21, solve_1(example.to_owned()))
    }
}
