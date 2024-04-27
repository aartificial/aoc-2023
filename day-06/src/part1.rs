use crate::custom_error::AocError;
use itertools::Itertools;
use std::iter::zip;

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

fn parse_input(input: &str) -> Result<Vec<Race>, AocError> {
    let (times, distances) = input.split_once('\n').ok_or(AocError::ParseError)?;

    let times = times
        .strip_prefix("Time:")
        .ok_or(AocError::ParseError)?
        .trim();
    let distances = distances
        .strip_prefix("Distance:")
        .ok_or(AocError::ParseError)?
        .trim();

    let mut races = Vec::with_capacity(times.len());

    let times = times
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let distances = distances
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    for (time, distance) in zip(times, distances) {
        races.push(Race::new(time, distance))
    }

    Ok(races)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let races = parse_input(input)?;

    let result = races
        .iter()
        .map(|race| {
            let mut wins = 0;
            for hold in 0..=race.time {
                if race.is_winning(hold) {
                    wins += 1;
                }
            }

            wins
        })
        .product();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
