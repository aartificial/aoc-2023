use crate::custom_error::AocError;
use crate::shared::parse_input;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Universe {
    height: usize,
    width: usize,
    expansion_rate: usize,
    galaxies: Vec<Galaxy>,
}

impl Universe {
    pub(crate) fn new(
        galaxies: Vec<Galaxy>,
        height: usize,
        width: usize,
        expansion_rate: usize,
    ) -> Self {
        let mut universe = Self {
            galaxies,
            height,
            width,
            expansion_rate,
        };
        universe.expand();
        universe
    }

    pub(crate) fn calculate_distances(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                sum += Self::distance(
                    self.galaxies[i].x as i64,
                    self.galaxies[i].y as i64,
                    self.galaxies[j].x as i64,
                    self.galaxies[j].y as i64,
                );
            }
        }
        sum
    }

    fn expand(&mut self) {
        let x_coords = self.galaxies.iter().map(|c| c.x).collect_vec();
        let y_coords = self.galaxies.iter().map(|c| c.y).collect_vec();

        let x_expanded = (0..self.width)
            .filter(|i| !x_coords.contains(i))
            .collect_vec();

        let y_expanded = (0..self.height)
            .filter(|i| !y_coords.contains(i))
            .collect_vec();

        let x_gt = |x: u64| {
            x_expanded
                .iter()
                .copied()
                .filter(|x_ex| x > *x_ex as u64)
                .count()
        };
        let y_gt = |y: u64| {
            y_expanded
                .iter()
                .copied()
                .filter(|y_ex| y > *y_ex as u64)
                .count()
        };

        self.galaxies.iter_mut().for_each(|coord| {
            coord.x += x_gt(coord.x as u64) * self.expansion_rate;
            coord.y += y_gt(coord.y as u64) * self.expansion_rate;
        });
    }

    fn distance(x1: i64, y1: i64, x2: i64, y2: i64) -> usize {
        ((x2 - x1).abs() + (y2 - y1).abs()) as usize
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let universe = Universe::new(parse_input(input), height, width, 1);
    Ok(universe.calculate_distances())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, process(input)?);
        Ok(())
    }
}
