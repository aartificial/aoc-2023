use crate::custom_error::AocError;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(TileType::from).collect())
        .collect_vec()
        .to_grid();

    Ok(grid.energize(0, 0, Direction::Right))
}

impl Grid {
    pub fn energize(&self, x: usize, y: usize, dir: Direction) -> usize {
        let mut memo: HashSet<Beam> = HashSet::new();
        let mut energized = HashSet::new();
        let mut queue = VecDeque::from([Beam {
            pos: Point { x, y },
            dir,
        }]);

        while let Some(beam) = queue.pop_front() {
            match memo.contains(&beam) {
                true => continue,
                false => memo.insert(beam.clone()),
            };

            energized.insert(beam.pos);
            self.spread_beam(&mut queue, &beam);
        }

        energized.len()
    }

    fn reflect_beam(&self, dir: &Direction, x: &usize, y: &usize) -> Vec<Direction> {
        match (&self.tiles[*y][*x], dir) {
            (TileType::Empty, dir) => vec![dir.clone()],
            (TileType::Mirror(mirror_type), dir) => match (mirror_type, dir) {
                (MirrorType::ForwardSlash, Direction::Up)
                | (MirrorType::BackwardSlash, Direction::Down) => vec![Direction::Right],
                (MirrorType::BackwardSlash, Direction::Right)
                | (MirrorType::ForwardSlash, Direction::Left) => vec![Direction::Down],
                (MirrorType::ForwardSlash, Direction::Down)
                | (MirrorType::BackwardSlash, Direction::Up) => vec![Direction::Left],
                (MirrorType::BackwardSlash, Direction::Left)
                | (MirrorType::ForwardSlash, Direction::Right) => vec![Direction::Up],
                (MirrorType::Vertical, _) => vec![Direction::Up, Direction::Down],
                (MirrorType::Horizontal, _) => vec![Direction::Left, Direction::Right],
            },
        }
    }

    fn spread_beam(&self, queue: &mut VecDeque<Beam>, beam: &Beam) {
        for dir in self.reflect_beam(&beam.dir, &beam.pos.x, &beam.pos.y) {
            let b = Beam {
                pos: Point {
                    x: beam.pos.x,
                    y: beam.pos.y,
                },
                dir: dir.clone(),
            };
            if let Some(beam) = b.forward(self.tiles[0].len(), self.tiles.len()) {
                queue.push_back(beam);
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Beam {
    pos: Point,
    dir: Direction,
}

impl Beam {
    fn forward(mut self, x: usize, y: usize) -> Option<Self> {
        match self.dir {
            Direction::Up if self.pos.y > 0 => self.pos.y -= 1,
            Direction::Down if self.pos.y < y - 1 => self.pos.y += 1,
            Direction::Left if self.pos.x > 0 => self.pos.x -= 1,
            Direction::Right if self.pos.x < x - 1 => self.pos.x += 1,
            _ => return None,
        }
        Some(self)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Grid {
    pub(crate) tiles: Vec<Vec<TileType>>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub trait IntoGrid {
    fn to_grid(self) -> Grid;
}

impl IntoGrid for Vec<Vec<TileType>> {
    fn to_grid(self) -> Grid {
        Grid { tiles: self }
    }
}

#[derive(Debug)]
pub enum TileType {
    Empty,
    Mirror(MirrorType),
}

#[derive(Debug)]
pub enum MirrorType {
    ForwardSlash,
    BackwardSlash,
    Vertical,
    Horizontal,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '|' => TileType::Mirror(MirrorType::Vertical),
            '-' => TileType::Mirror(MirrorType::Horizontal),
            '/' => TileType::Mirror(MirrorType::ForwardSlash),
            '\\' => TileType::Mirror(MirrorType::BackwardSlash),
            _ => TileType::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use tracing_subscriber::fmt;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn parse() -> miette::Result<()> {
        INPUT
            .lines()
            .map(|line| line.chars().map(TileType::from).collect_vec())
            .collect_vec()
            .to_grid();
        Ok(())
    }

    #[test]
    fn energize() {
        fmt::init();
        let grid = INPUT
            .lines()
            .map(|line| line.chars().map(TileType::from).collect_vec())
            .collect_vec()
            .to_grid();

        assert_eq!(grid.energize(0, 0, Direction::Right), 46);
    }
}
