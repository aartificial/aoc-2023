use crate::custom_error::AocError;
use crate::shared::{Notes, Reflection};
use itertools::Itertools;
use std::str::FromStr;

pub struct Pattern {
    pub cols: Vec<String>,
    pub rows: Vec<String>,
}

impl FromStr for Pattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().map(|line| line.to_string()).collect_vec();
        let cols = (0..rows[0].len())
            .map(|col| {
                rows.iter()
                    .map(|row| row.chars().nth(col).unwrap())
                    .collect::<String>()
            })
            .collect_vec();

        Ok(Self { cols, rows })
    }
}

impl Reflection for Pattern {
    fn is_perfect_reflection(pattern: &[String], coords: (usize, usize)) -> bool {
        let mut cols = coords.0;
        let mut rows = coords.1;
        loop {
            if pattern[cols] != pattern[rows] {
                return false;
            }

            if cols == 0 || rows == pattern.len() - 1 {
                break;
            }

            cols -= 1;
            rows += 1;
        }
        true
    }

    fn find_reflection(pattern: &[String]) -> usize {
        let mut cols = 0;
        let mut rows = 1;

        while rows < pattern.len() {
            if pattern[cols] == pattern[rows] && Self::is_perfect_reflection(pattern, (cols, rows))
            {
                return rows;
            }
            cols += 1;
            rows += 1;
        }

        0
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    Ok(input
        .parse::<Notes<Pattern>>()
        .unwrap()
        .patterns
        .iter()
        .map(|pattern| {
            let cols = Pattern::find_reflection(&pattern.cols);
            let rows = Pattern::find_reflection(&pattern.rows);

            assert!(cols == 0 || rows == 0, "Pattern is not a reflection");
            assert!(cols != 0 || rows != 0, "Pattern is not a reflection");

            cols + (100 * rows)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(405, process(input)?);
        Ok(())
    }
}
