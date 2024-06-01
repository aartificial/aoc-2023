use crate::custom_error::AocError;
use crate::part1::{Area, Command, Dir};

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_suffix(')').unwrap();
            let (_, hex) = line.split_once("(#").unwrap();
            let (amount, dir) = hex.split_at(5);
            let amount = usize::from_str_radix(amount, 16).unwrap();
            let dir = match dir {
                "3" => Dir::Up,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "0" => Dir::Right,
                _ => panic!("invalid direction"),
            };
            Command::new(dir, amount)
        })
        .collect()
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
        assert_eq!(952408144115, process(input)?);
        Ok(())
    }
}
