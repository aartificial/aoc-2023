use crate::part1::Galaxy;
use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<Galaxy> {
    let galaxies = input
        .lines()
        .map(|line| line.to_string())
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| Galaxy::new(i, j))
                .collect_vec()
        })
        .collect_vec();
    galaxies
}
