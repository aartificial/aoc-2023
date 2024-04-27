use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
struct Card(char);

impl Deref for Card {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Card {
    fn new(card: char) -> Self {
        Self(card)
    }

    fn value(&self) -> usize {
        match self.0 {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card rank"),
        }
    }
}

impl Eq for Card {}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Game {
    hands: Vec<Hand>,
}

#[derive(Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    rank: HandRank,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
enum HandRank {
    GHighCard,
    FPair,
    ETwoPair,
    DThreeOfAKind,
    CFullHouse,
    BFourOfAKind,
    AFiveOfAKind,
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Self {
        let cards = cards.chars().map(Card::new).collect_vec();

        let rank = Self::rank(&cards);

        Self { cards, bid, rank }
    }

    fn rank(cards: &[Card]) -> HandRank {
        let mut counts = [0; 15];

        for c in cards {
            counts[c.value()] += 1;
        }

        let mut pairs = 0;
        let mut threes = 0;
        let mut fours = 0;
        let mut fives = 0;

        for count in counts.iter() {
            match count {
                2 => pairs += 1,
                3 => threes += 1,
                4 => fours += 1,
                5 => fives += 1,
                _ => (),
            }
        }

        match (pairs, threes, fours, fives) {
            (0, 0, 0, 0) => HandRank::GHighCard,
            (1, 0, 0, 0) => HandRank::FPair,
            (2, 0, 0, 0) => HandRank::ETwoPair,
            (0, 1, 0, 0) => HandRank::DThreeOfAKind,
            (1, 1, 0, 0) => HandRank::CFullHouse,
            (0, 0, 1, 0) => HandRank::BFourOfAKind,
            (0, 0, 0, 1) => HandRank::AFiveOfAKind,
            _ => unreachable!("invalid hand"),
        }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", String::from_iter(self.cards.iter().map(|w| w.0)))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    fn new(hands: Vec<Hand>) -> Self {
        Self { hands }
    }

    fn total_winnings(&self) -> u32 {
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hand)| {
                let multiplier = i as u32 + 1;
                hand.bid * multiplier
            })
            .sum()
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            Hand::new(cards, bid.parse().unwrap())
        })
        .sorted()
        .collect_vec();

    let game = Game::new(hands);

    Ok(game.total_winnings())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input)?);
        Ok(())
    }
}
