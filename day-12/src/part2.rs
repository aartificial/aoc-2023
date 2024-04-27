use std::iter::repeat;
use std::str::FromStr;

use crate::custom_error::AocError;
use crate::part1::{possible_arrangements, UNKNOWN};
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug)]
struct Record {
    springs: String,
    groups: Vec<usize>,
}

impl FromStr for Record {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let result = value.split_once(' ').ok_or(());
        let (springs, groups) = match result {
            Ok(result) => result,
            Err(_) => return Err("Invalid record format."),
        };

        let springs = repeat(springs).take(5).collect_vec().join(UNKNOWN);
        let groups = groups
            .split(',')
            .map(|value| {
                value
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse info value.")
                    .unwrap()
            })
            .collect_vec()
            .repeat(5);

        Ok(Self { springs, groups })
    }
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

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    compute(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::part1::possible_arrangements;
    use crate::part2::{process, Record};

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
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
        assert_eq!(525152, process(input)?);
        Ok(())
    }
}
