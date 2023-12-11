use std::{fmt::Error, fs, str::FromStr};

pub const RED_BOUND: u32 = 12;
pub const GREEN_BOUND: u32 = 13;
pub const BLUE_BOUND: u32 = 14;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").unwrap();
    let mut sum = 0;
    for c in puzzle_input.lines() {
        let game = Game::from_str(c).unwrap();
        if game.rounds.iter().all(|round| {
            round.green <= GREEN_BOUND && round.red <= RED_BOUND && round.blue <= BLUE_BOUND
        }) {
            sum += game.id
        }
    }
    println!("{}", sum);
}

pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

pub struct Round {
    pub green: u32,
    pub red: u32,
    pub blue: u32,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = Round {
            green: 0,
            red: 0,
            blue: 0,
        };
        for color_draw in s.split(',') {
            let (value, color) = color_draw.trim().split_once(' ').unwrap();
            let value = value.parse().unwrap();
            match color {
                "green" => round.green = value,
                "red" => round.red = value,
                "blue" => round.blue = value,
                _ => panic!(),
            }
        }
        Ok(round)
        
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // extract ID
        let mut splitter = s.split(':');
        let id = splitter.next().unwrap()[5..].parse().unwrap();
        let game_string = splitter.next().unwrap();
        let mut game = Self {
            id,
            rounds: Vec::new(),
        };
        for round_str in game_string.split(';') {
            game.rounds.push(Round::from_str(round_str).unwrap())
        }
        Ok(game)
    }
}
