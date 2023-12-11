use std::{fmt::Error, fs, str::FromStr};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();

    let cards = puzzle_input
        .lines()
        .map(|x| Card::from_str(x.split(':').last().unwrap()).unwrap());

    let win: Vec<usize> = cards.map(|x| x.compute_winnings()).collect();
    let mut dyn_win: Vec<usize> = win.iter().map(|_x| 1).collect();

    for (id, value) in win.iter().enumerate().rev() {
        for i in 0..(*value) {
            dyn_win[id] += dyn_win[id + i + 1]
        }
    }

    let sum: usize = dyn_win.iter().sum();
    println!("{sum}")
}

#[derive(Debug)]
pub struct Card {
    winning: Vec<usize>,
    own_numbers: Vec<usize>,
}

impl Card {
    pub fn compute_winnings(&self) -> usize {
        let matches = self
            .winning
            .iter()
            .filter(|x| self.own_numbers.contains(x))
            .count();
        matches
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_str, own_string) = s.split_once('|').unwrap();
        let winning: Vec<usize> = winning_str
            .split_whitespace()
            .filter_map(|x| (*x).parse().ok())
            .collect();
        let own_numbers = own_string
            .split_whitespace()
            .filter_map(|x| (*x).parse().ok())
            .collect();
        Ok(Card {
            winning,
            own_numbers,
        })
    }
}
