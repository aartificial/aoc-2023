use crate::custom_error::AocError;
use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn is_winning(&self, hold: u64) -> bool {
        let remaining = self.time - hold;
        let distance_covered = remaining * hold;

        distance_covered > self.distance
    }
}

fn parse_input(input: &str) -> Result<Race, AocError> {
    let (times, distances) = input.split_once('\n').ok_or(AocError::ParseError)?;

    let times = times
        .strip_prefix("Time:")
        .ok_or(AocError::ParseError)?
        .trim();
    let distances = distances
        .strip_prefix("Distance:")
        .ok_or(AocError::ParseError)?
        .trim();

    let time: u64 = times.split_whitespace().join("").parse().unwrap();
    let distance = distances.split_whitespace().join("").parse().unwrap();

    Ok(Race::new(time, distance))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let race = parse_input(input)?;

    let mut wins = 0;
    for hold in 0..=race.time {
        if race.is_winning(hold) {
            wins += 1;
        }
    }

    Ok(wins)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt;

    #[test]
    fn test_process() -> miette::Result<()> {
        fmt::init();
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
