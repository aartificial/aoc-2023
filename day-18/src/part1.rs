use crate::custom_error::AocError;

pub struct Command {
    dir: Dir,
    amount: usize,
}

impl Command {
    pub fn new(dir: Dir, amount: usize) -> Self {
        Self { dir, amount }
    }
}

struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn next(&self, dir: &Dir, amount: usize) -> Self {
        match dir {
            Dir::Up => Self::new(self.x, self.y + amount as i64),
            Dir::Down => Self::new(self.x, self.y - amount as i64),
            Dir::Left => Self::new(self.x - amount as i64, self.y),
            Dir::Right => Self::new(self.x + amount as i64, self.y),
        }
    }
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let (c, _) = line.split_once(" (").unwrap();
            let (_, amount) = c.split_once(' ').unwrap();
            let dir = match line.chars().next().unwrap() {
                'U' => Dir::Up,
                'D' => Dir::Down,
                'L' => Dir::Left,
                'R' => Dir::Right,
                _ => panic!("invalid direction"),
            };
            let amount = amount.parse().unwrap();
            Command { dir, amount }
        })
        .collect()
}

pub trait Area {
    fn area(&self) -> usize;
}

impl Area for Vec<Command> {
    fn area(&self) -> usize {
        let (area, perimeter, _) = self.iter().fold(
            (0, 0, Vec2::new(0, 0)),
            |(area, perimeter, pos), Command { dir, amount }| {
                let new = pos.next(dir, *amount);
                let area = area + (pos.x * new.y) - (pos.y * new.x);
                let perimeter = perimeter + (pos.x - new.x).abs() + (pos.y - new.y).abs();
                (area, perimeter, new)
            },
        );

        ((area.abs() + perimeter) / 2 + 1) as usize
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    Ok(parse(input).area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        assert_eq!(62, process(input)?);
        Ok(())
    }
}
