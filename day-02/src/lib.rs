use std::num::ParseIntError;
use std::str::FromStr;

pub mod custom_error;

pub mod part1;
pub mod part2;

const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;
const MAX_RED: u32 = 12;

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    draw: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    color: Color,
    number: u32,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Score {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
pub enum GameError {
    Number(ParseIntError),
    InvalidColor(String),
    Color,
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let id = parts[0][5..].trim().parse()?;
        let sets = parts[1]
            .split(';')
            .map(|set| set.parse())
            .collect::<Result<Vec<Set>, _>>()?;

        Ok(Game { id, sets })
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

impl From<ParseIntError> for GameError {
    fn from(error: ParseIntError) -> Self {
        GameError::Number(error)
    }
}

impl From<()> for GameError {
    fn from(_: ()) -> Self {
        GameError::Color
    }
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

impl Game {
    fn possible(&self) -> bool {
        self.sets.iter().all(|set| set.possible())
    }
}

impl Set {
    fn possible(&self) -> bool {
        let Score { red, green, blue } = self.draw.iter().fold(
            Score {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut scores, card| {
                match card.color {
                    Color::Red => scores.red += card.number,
                    Color::Green => scores.green += card.number,
                    Color::Blue => scores.blue += card.number,
                };
                scores
            },
        );

        red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE
    }
}
