use crate::color::Color;
use crate::custom_error::GameError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    pub color: Color,
    pub number: u32,
}

impl FromStr for Card {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let number = parts[0].parse()?;
        let color = parts[1].parse()?;

        Ok(Card { number, color })
    }
}
