use crate::custom_error::AocError;
use crate::Game;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let result = input
        .lines()
        .map(|s| s.parse::<Game>().unwrap())
        .filter(|game| game.possible())
        .map(|game| game.id)
        .sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
