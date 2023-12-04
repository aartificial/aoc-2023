use crate::custom_error::CardError;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    pub winners: Vec<u32>,
    pub hand: Vec<u32>,
    pub repetitions: u32,
}

impl Card {
    pub fn points(&self) -> u32 {
        let count = self.winning_count();
        match count {
            0 => 0,
            _ => 2_u32.pow(count as u32 - 1),
        }
    }

    pub fn winning_count(&self) -> usize {
        self.hand
            .iter()
            .filter(|n| self.is_winning_number(**n))
            .count()
    }

    fn is_winning_number(&self, number: u32) -> bool {
        self.winners.contains(&number)
    }
}

impl FromStr for Card {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();

        if parts.len() != 2 {
            return Err(CardError::Parts);
        }

        let winners_res: Result<Vec<u32>, ParseIntError> = parts[0]
            .split_whitespace()
            .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
            .map(|s| s.parse::<u32>())
            .collect();
        let hand_res: Result<Vec<u32>, ParseIntError> = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect();

        match (winners_res, hand_res) {
            (Ok(winners), Ok(hand)) => Ok(Card {
                winners,
                hand,
                repetitions: 1,
            }),
            _ => Err(CardError::Number),
        }
    }
}
