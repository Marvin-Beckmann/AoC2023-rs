use std::{collections::HashMap, fs, os::unix::ffi::OsStrExt, str::from_utf8};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
    let mut boxes = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new())
    }

    for str in puzzle_input.split(',') {
        if let Some(pos_new) = str.chars().position(|x| x == '=') {
            let label = &str[..pos_new];

            let number: usize = str[(pos_new + 1)..].parse().unwrap();
            if let Some(index) = boxes[compute_hash(label)]
                .iter()
                .position(|(label_old, _)| &label == label_old)
            {
                boxes[compute_hash(label)][index] = (label, number);
            } else {
                boxes[compute_hash(label)].push((label, number))
            }
        } else {
            let pos_minus = str.chars().position(|x| x == '-').unwrap();
            let label = &str[..pos_minus];
            if let Some(index) = boxes[compute_hash(label)]
                .iter()
                .position(|(label_old, _)| &label == label_old)
            {
                boxes[compute_hash(label)].remove(index);
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(id, box_id)| -> usize {
            (id + 1)
                * box_id
                    .iter()
                    .enumerate()
                    .map(|(slot, elem)| (slot + 1) * elem.1)
                    .sum::<usize>()
        })
        .sum()
}

pub fn compute_hash(str: &str) -> usize {
    str.chars()
        .fold(0, |acc, char| 17 * (acc + (char as usize)) % 256)
}

#[cfg(test)]
mod test {
    use crate::solve_2;

    #[test]
    fn test() {
        let example = "rn=1";

        assert_eq!(1, solve_2(example.to_owned()))
    }
}
