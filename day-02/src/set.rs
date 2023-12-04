use crate::card::Card;
use crate::color::Color;
use crate::custom_error::GameError;
use crate::score::Score;
use crate::{MAX_BLUE, MAX_GREEN, MAX_RED};
use std::str::FromStr;

#[derive(Debug)]
pub struct Set {
    draw: Vec<Card>,
}

impl Set {
    pub fn calculate_color_totals(&self) -> Score {
        let mut scores = Score::default();
        for card in &self.draw {
            match card.color {
                Color::Red => scores.red += card.number,
                Color::Green => scores.green += card.number,
                Color::Blue => scores.blue += card.number,
            };
        }
        scores
    }

    pub fn is_possible(&self) -> bool {
        let Score { red, green, blue } = self.calculate_color_totals();
        red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE
    }
}

impl FromStr for Set {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let draw = s
            .split(',')
            .map(|card| card.trim().parse())
            .collect::<Result<Vec<Card>, _>>()?;

        Ok(Set { draw })
    }
}
