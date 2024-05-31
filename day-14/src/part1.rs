use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect_vec();

    let total = total_load(&mut grid);

    Ok(total)
}

fn total_load(grid: &mut [Vec<char>]) -> usize {
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == ROUNDED {
                total += roll_north(grid, x, y);
            }
        }
    }

    total
}

fn roll_north(grid: &mut [Vec<char>], x: usize, y: usize) -> usize {
    let mut pos = y;

    for y in (1..=y).rev() {
        match grid[y - 1][x] {
            CUBE | ROUNDED => break,
            _ => {
                grid[y - 1][x] = ROUNDED;
                grid[y][x] = EMPTY;
                pos -= 1;
            }
        }
    }

    grid.len() - pos
}

const ROUNDED: char = 'O';
const CUBE: char = '#';
const EMPTY: char = '.';

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(136, process(input)?);
        Ok(())
    }
}
