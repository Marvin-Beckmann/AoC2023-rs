use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> usize {
    let mut total_field: Vec<Vec<Field>> = puzzle_input
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect();

    let mut light = vec![(0, 0, Direction::West)];
    while !light.is_empty() {
        let mut tmp_light = Vec::new();
        for (y, x, prev_dir) in &light {
            let y = *y;
            let x = *x;
            if y >= total_field.len() || x >= total_field.len() {
                continue;
            }
            if let Some((dir, dir_opt)) = total_field[y][x].energy_passing_from(prev_dir) {
                let mut directions = vec![dir];
                if let Some(dir2) = dir_opt {
                    directions.push(dir2)
                }
                for new_direction in directions {
                    match new_direction {
                        Direction::East => {
                            if x < total_field.len() - 1 {
                                tmp_light.push((y, x + 1, new_direction.opposite()))
                            }
                        }
                        Direction::North => {
                            if y != 0 {
                                tmp_light.push((y - 1, x, new_direction.opposite()))
                            }
                        }
                        Direction::South => {
                            if y < total_field[0].len() - 1 {
                                tmp_light.push((y + 1, x, new_direction.opposite()))
                            }
                        }
                        Direction::West => {
                            if x != 0 {
                                tmp_light.push((y, x - 1, new_direction.opposite()))
                            }
                        }
                    }
                }
            };
        }
        light = tmp_light;
    }
    total_field
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| if x.compute_energy() > 0 { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

pub struct Field {
    symbol: Symbol,
    energy_passing_from_north: bool,
    energy_passing_from_east: bool,
    energy_passing_from_south: bool,
    energy_passing_from_west: bool,
}

#[derive(PartialEq)]
pub enum Symbol {
    Wall,
    HorizontalSplitter,
    VerticalSplitter,
    TopBottom,
    BottomTop,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Wall,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '\\' => Self::TopBottom,
            '/' => Self::BottomTop,
            _ => panic!(),
        }
    }
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        Field {
            symbol: Symbol::from(value),
            energy_passing_from_north: false,
            energy_passing_from_east: false,
            energy_passing_from_south: false,
            energy_passing_from_west: false,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl Field {
    pub fn energy_passing_from(
        &mut self,
        dir: &Direction,
    ) -> Option<(Direction, Option<Direction>)> {
        match dir {
            Direction::North => {
                if self.energy_passing_from_north {
                    return None;
                } else {
                    self.energy_passing_from_north = true
                }
            }
            Direction::South => {
                if self.energy_passing_from_south {
                    return None;
                } else {
                    self.energy_passing_from_south = true
                }
            }
            Direction::East => {
                if self.energy_passing_from_east {
                    return None;
                } else {
                    self.energy_passing_from_east = true
                }
            }
            Direction::West => {
                if self.energy_passing_from_west {
                    return None;
                } else {
                    self.energy_passing_from_west = true
                }
            }
        };
        Some(match self.symbol {
            Symbol::Wall => (dir.opposite(), None),
            Symbol::HorizontalSplitter => {
                if *dir == Direction::North || *dir == Direction::South {
                    (Direction::East, Some(Direction::West))
                } else {
                    (dir.opposite(), None)
                }
            }
            Symbol::VerticalSplitter => {
                if *dir == Direction::East || *dir == Direction::West {
                    (Direction::North, Some(Direction::South))
                } else {
                    (dir.opposite(), None)
                }
            }
            Symbol::TopBottom => (
                match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                },
                None,
            ),
            Symbol::BottomTop => (
                match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                },
                None,
            ),
        })
    }

    pub fn compute_energy(&self) -> usize {
        let mut count = 0;
        for dir in [
            self.energy_passing_from_east,
            self.energy_passing_from_north,
            self.energy_passing_from_south,
            self.energy_passing_from_west,
        ] {
            if dir {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use crate::solve_1;

    #[test]
    fn test() {
        let example = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";

        assert_eq!(46, solve_1(example.to_owned()))
    }
}
