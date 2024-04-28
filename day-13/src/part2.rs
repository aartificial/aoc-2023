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

fn ne(pattern: &[String], cols: usize, rows: usize) -> usize {
    pattern[cols]
        .chars()
        .zip(pattern[rows].chars())
        .filter(|(a, b)| a != b)
        .collect_vec()
        .len()
}

impl Reflection for Pattern {
    fn is_perfect_reflection(pattern: &[String], coords: (usize, usize)) -> bool {
        let mut cols = coords.0;
        let mut rows = coords.1;
        let mut mismatched = 0;

        loop {
            if pattern[cols] != pattern[rows] {
                if ne(pattern, cols, rows) > 1 {
                    return false;
                } else {
                    mismatched += 1;
                }

                if mismatched > 1 {
                    return false;
                }
            }

            if cols == 0 || rows == pattern.len() - 1 {
                break;
            }

            cols -= 1;
            rows += 1;
        }

        mismatched == 1
    }

    fn find_reflection(pattern: &[String]) -> usize {
        let mut cols = 0;
        let mut rows = 1;

        while rows < pattern.len() {
            if ne(pattern, cols, rows) < 2 && Self::is_perfect_reflection(pattern, (cols, rows)) {
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
        assert_eq!(400, process(input)?);
        Ok(())
    }
}
