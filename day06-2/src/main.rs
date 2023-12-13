use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let (time, distance) = puzzle_input.split_once('\n').unwrap();
    let time: usize = time
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: usize = distance
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let (lower, upper) = compute_roots_quadratic_function(time, distance);

    println!("{}", upper.floor() - lower.ceil() + 1.0);
}

/// 'charge_time*(time - charge_time) - distance -1'
/// = '-charge_time^2 + time*charge_time - distance -1'
/// = '- (charge_time^2 - time*charge_time + distance + 1)'
pub fn compute_roots_quadratic_function(time: usize, distance: usize) -> (f64, f64) {
    let discriminant = ((time * time - 4 * (distance + 1)) as f64).sqrt();
    let lower = ((time as f64 - discriminant) / 2.0).ceil();
    let upper = ((time as f64 + discriminant) / 2.0).floor();

    (lower, upper)
}
