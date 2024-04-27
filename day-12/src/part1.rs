use derive_more::Into;
use std::fmt::Debug;
use std::ops::Range;
use std::str::FromStr;

use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    compute(input)
}

pub fn compute(input: &str) -> Result<usize, AocError> {
    let records = input
        .lines()
        .map(|line| line.parse::<Record>().unwrap())
        .collect_vec();

    let result = records
        .par_iter()
        .map(|r| possible_arrangements(r.springs.clone(), r.groups.clone()))
        .sum();

    Ok(result)
}

const DAMAGED: &str = "#";
const DAMAGED_CHAR: char = '#';
const OPERATIONAL: &str = ".";
pub const UNKNOWN: &str = "?";
const UNKNOWN_CHAR: char = '?';

#[derive(Debug, Into)]
struct Record {
    pub springs: String,
    pub groups: Vec<usize>,
}

impl FromStr for Record {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let result = value.split_once(' ').ok_or(());
        let (records, info) = match result {
            Ok(result) => result,
            Err(_) => return Err("Invalid record format."),
        };

        let springs = records.chars().collect();
        let info = info
            .split(',')
            .map(|value| {
                value
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse info value.")
                    .unwrap()
            })
            .collect_vec();

        Ok(Self {
            springs,
            groups: info,
        })
    }
}

pub(crate) fn possible_arrangements(springs: String, groups: Vec<usize>) -> usize {
    let mut dp = vec![vec![0; groups.len()]; springs.len() + groups[groups.len() - 1] + 1];
    let mut min_j = 0;
    'main: for i in 0..springs.len() {
        if i > 0 {
            dp[i - 1].clear()
        }
        for j in 0..groups.len() {
            let current = &springs[i..i + 1];
            if j < min_j {
                continue;
            }
            match current {
                OPERATIONAL => continue,
                DAMAGED if j == 0 => min_j = 1,
                _ => (),
            };
            if current == OPERATIONAL {
                continue 'main;
            }
            if is_valid(&springs, &groups, &dp, i, j) {
                continue;
            }
            update_dp(&springs, &groups, &mut dp, i, j);
        }
    }
    extract_result(&dp)
}

fn extract_result(dp: &[Vec<usize>]) -> usize {
    dp[dp.len() - 1][dp[dp.len() - 1].len() - 1]
}

fn update_dp(springs: &str, groups: &[usize], dp: &mut [Vec<usize>], i: usize, j: usize) {
    for k in next_indices(springs, groups, dp, i, j) {
        match j > 0 {
            true => dp[k][j] += dp[i][j - 1],
            false => dp[k][j] += 1,
        }
    }
}

fn is_valid(springs: &str, groups: &[usize], dp: &[Vec<usize>], i: usize, j: usize) -> bool {
    !can_be_placed(dp, i, j)
        || !group_fits_in_spring(springs, groups, i, j)
        || (is_last_group(groups, j) && springs_remaining(springs, groups, i, j))
        || !is_group_valid(springs, groups, i, j)
}

fn next_indices(
    springs: &str,
    groups: &[usize],
    dp: &[Vec<usize>],
    i: usize,
    j: usize,
) -> Range<usize> {
    let next_start_idx = springs.len().min(i + groups[j] + 1);
    let next_broken_idx = match springs[next_start_idx..].find(DAMAGED) {
        Some(n) => next_start_idx + n,
        None => dp.len() - 1,
    };
    Range {
        start: next_start_idx,
        end: next_broken_idx + 1,
    }
}

fn is_group_valid(springs: &str, groups: &[usize], i: usize, j: usize) -> bool {
    let max_index = springs.len().min(i + groups[j]);
    let end_reached = max_index == springs.len();
    let subsequent_character = springs.get(max_index..max_index + 1).unwrap_or("");
    let valid = springs[i..i + groups[j]]
        .chars()
        .all(|x| x == UNKNOWN_CHAR || x == DAMAGED_CHAR)
        && (end_reached || subsequent_character != DAMAGED);
    valid
}

fn springs_remaining(springs: &str, groups: &[usize], i: usize, j: usize) -> bool {
    springs[i + groups[j]..].chars().any(|c| c == DAMAGED_CHAR)
}

const fn is_last_group(groups: &[usize], j: usize) -> bool {
    j == groups.len() - 1
}

fn group_fits_in_spring(springs: &str, groups: &[usize], i: usize, j: usize) -> bool {
    groups[j..].iter().sum::<usize>() + groups[j..].len() - 1 <= springs[i..].len()
}

fn can_be_placed(dp: &[Vec<usize>], i: usize, j: usize) -> bool {
    !(j > 0 && dp[i][j - 1] == 0)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_permutations(#[case] input: &str, #[case] expected: usize) {
        let record = input.parse::<Record>().unwrap();
        assert_eq!(
            expected,
            possible_arrangements(record.springs, record.groups)
        )
    }

    #[test]
    fn base_test() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(21, process(input)?);
        Ok(())
    }
}
