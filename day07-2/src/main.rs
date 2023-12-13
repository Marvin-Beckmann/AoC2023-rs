use enum_iterator::{all, Sequence};
use std::{fmt::Error, fs, str::FromStr};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    let mut hands: Vec<Hand> = puzzle_input
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect();
    hands.sort();

    let total_winnings: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    println!("{}", total_winnings)
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<CardType>,
    pub bid: usize,
    pub kind: TYPE,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<CardType> = cards.chars().map(CardType::from).collect();

        Ok(Self {
            kind: TYPE::from(cards.clone()),
            cards,
            bid: bid.parse().unwrap(),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {
                for i in 0..5 {
                    let cmp = self.cards[i].partial_cmp(&other.cards[i]);
                    if cmp != Some(core::cmp::Ordering::Equal) {
                        return cmp;
                    }
                }
            }
            ord => return ord,
        }
        Some(core::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TYPE {
    HighCard,
    OnePair,
    TwoPair,
    ThreeAKind,
    FullHouse,
    FourAKind,
    FiveAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Sequence, Clone)]
pub enum CardType {
    Joker,
    TWO,
    THREE,
    FOUR,
    FIVE,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for CardType {
    fn from(value: char) -> Self {
        match value {
            '2' => CardType::TWO,
            '3' => CardType::THREE,
            '4' => CardType::FOUR,
            '5' => CardType::FIVE,
            '6' => CardType::Six,
            '7' => CardType::Seven,
            '8' => CardType::Eight,
            '9' => CardType::Nine,
            'T' => CardType::Ten,
            'J' => CardType::Joker,
            'Q' => CardType::Queen,
            'K' => CardType::King,
            'A' => CardType::Ace,
            _ => panic!(),
        }
    }
}

impl From<Vec<CardType>> for TYPE {
    fn from(cards: Vec<CardType>) -> Self {
        assert_eq!(cards.len(), 5);

        if cards.contains(&CardType::Joker) {
            let first_joker = cards.iter().position(|x| x == &CardType::Joker).unwrap();
            return all::<CardType>()
                .filter(|kind| kind != &CardType::Joker)
                .map(|replace_type| {
                    let mut cards_new = cards.clone();
                    cards_new[first_joker] = replace_type;
                    TYPE::from(cards_new)
                })
                .max()
                .unwrap();
        }

        let string_match = cards
            .iter()
            .map(|char| cards.iter().filter(|char2| char2 == &char).count());

        // check if 4 a kind or five a kind
        match string_match.clone().max().unwrap() {
            1 => TYPE::HighCard,
            2 => match string_match.filter(|x| x == &2).count() / 2 {
                1 => TYPE::OnePair,
                2 => TYPE::TwoPair,
                _ => panic!(),
            },
            3 => match &string_match.min().unwrap() {
                1 => TYPE::ThreeAKind,
                2 => TYPE::FullHouse,
                _ => panic!(),
            },
            4 => TYPE::FourAKind,
            5 => TYPE::FiveAKind,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TYPE;

    #[test]
    fn type_ordering() {
        assert!(TYPE::FiveAKind > TYPE::FourAKind);
        assert!(TYPE::FiveAKind > TYPE::FullHouse);
        assert!(TYPE::FiveAKind > TYPE::ThreeAKind);
        assert!(TYPE::FiveAKind > TYPE::TwoPair);
        assert!(TYPE::FiveAKind > TYPE::OnePair);
        assert!(TYPE::FiveAKind > TYPE::HighCard);

        assert_eq!(TYPE::FullHouse, TYPE::FullHouse)
    }
}
