use std::{fmt::Error, fs, str::FromStr};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();

    let cards = puzzle_input
        .lines()
        .map(|x| Card::from_str(x.split(':').last().unwrap()).unwrap());

    let win: usize = cards.map(|x| x.compute_winnings()).sum();
    println!("{win}")
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
        match matches {
            x if x > 0 => 2_usize.pow(x.try_into().unwrap()) / 2,
            _ => 0,
        }
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_str, own_string) = s.split_once('|').unwrap();
        let winning: Vec<usize> = winning_str
            .trim()
            .split(' ')
            .filter_map(|x| (*x).parse().ok())
            .collect();
        let own_numbers = own_string
            .trim()
            .split(' ')
            .filter_map(|x| (*x).parse().ok())
            .collect();
        Ok(Card {
            winning,
            own_numbers,
        })
    }
}
