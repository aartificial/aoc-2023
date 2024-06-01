pub mod custom_error;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Point {
    fn next(&self, dir: &Direction, r: usize, c: usize) -> Option<Self> {
        let new = match dir {
            Direction::Up if self.x > 0 => Self::new(self.x - 1, self.y),
            Direction::Down if self.x < (r - 1) => Self::new(self.x + 1, self.y),
            Direction::Left if self.y > 0 => Self::new(self.x, self.y - 1),
            Direction::Right if self.y < (c - 1) => Self::new(self.x, self.y + 1),
            _ => return None,
        };

        Some(new)
    }
}
