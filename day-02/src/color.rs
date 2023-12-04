use crate::custom_error::GameError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(GameError::InvalidColor(s.to_string())),
        }
    }
}
