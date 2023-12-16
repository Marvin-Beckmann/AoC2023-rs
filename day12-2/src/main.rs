use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> usize {
    let mut counter = 0;
    for line in puzzle_input.lines() {
        println!("round: {line}\n counter: {counter}");
        let (spring_records, check_sums) = line.split_once(' ').unwrap();
        let check_sums: Vec<usize> =
            format!("{}{}", check_sums, format!(",{}", check_sums).repeat(4))
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
        let spring_records = format!(
            "{}{}",
            spring_records,
            format!("?{}", spring_records).repeat(4)
        );
        counter +=
            compute_possible_solutions(spring_records.chars().collect(), &check_sums).unwrap();
    }
    counter
}

pub fn compute_possible_solutions(spring_records: Vec<char>, sums: &Vec<usize>) -> Option<usize> {
    // check for break criteria
    if sums.is_empty() {
        return match !spring_records.contains(&'#') {
            true => Some(1),
            false => None,
        };
    }

    let central_sum_id = sums.len() / 2;

    let left_groups = sums[..central_sum_id].to_vec();
    let right_groups = sums[central_sum_id + 1..].to_vec();
    // each group has needs an additional delimiter on top of its own elements
    let min_left_elems: usize = left_groups.iter().sum::<usize>() + left_groups.len();
    let min_right_elems: usize = right_groups.iter().sum::<usize>() + right_groups.len();

    let mut valid_assignments = 0;
    // find index where the central sum *could* start
    for potential_start in
        (min_left_elems)..=(spring_records.len() - min_right_elems - sums[central_sum_id])
    {
        let potential_end = potential_start + sums[central_sum_id];

        // all elements in central group have to be '#'
        // and check if the group is separated correctly
        let group_possible = spring_records[potential_start..potential_end]
            .iter()
            .all(|&x| x == '?' || x == '#');
        let left_delimiter =
            potential_start == 0 || ".?".contains(spring_records[potential_start - 1]);
        let right_delimiter =
            potential_end == spring_records.len() || ".?".contains(spring_records[potential_end]);

        if group_possible && left_delimiter && right_delimiter {
            let left_records = spring_records[..potential_start.max(1) - 1].to_vec();
            let right_records =
                spring_records[(potential_end + 1).min(spring_records.len())..].to_vec();
            // compute possible combinations for left and right assignments for fixed potential start and end
            if let Some(left_assignments) = compute_possible_solutions(left_records, &left_groups) {
                if let Some(right_assignments) =
                    compute_possible_solutions(right_records, &right_groups)
                {
                    valid_assignments += left_assignments * right_assignments
                }
            }
        }
    }
    Some(valid_assignments)
}

#[cfg(test)]
mod test {
    use crate::solve_2;

    #[test]
    fn test1() {
        let example_4 = "????.######..#####. 1,6,5";
        let example_5 = "?###???????? 3,2,1";

        assert_eq!(2500, solve_2(example_4.to_owned()));
        assert_eq!(506250, solve_2(example_5.to_owned()))
    }
}
