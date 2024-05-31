use crate::custom_error::AocError;
use crate::part1::{Direction, IntoGrid, TileType};
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(TileType::from).collect())
        .collect_vec()
        .to_grid();

    let max_energized = grid
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, _)| {
                match (x, y) {
                    (0, _) => Some(Direction::Right),
                    (_, 0) => Some(Direction::Down),
                    (_, y) if y == row.len() - 1 => Some(Direction::Up),
                    (x, _) if x == row.len() - 1 => Some(Direction::Left),
                    _ => None,
                }
                .map(|dir| (x, y, dir))
            })
        })
        .map(|(x, y, dir)| grid.energize(x, y, dir))
        .max();

    match max_energized {
        Some(max) => Ok(max),
        None => Ok(0),
    }
}
