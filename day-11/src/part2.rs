use crate::custom_error::AocError;
use crate::part1::Universe;
use crate::shared::parse_input;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let universe = Universe::new(parse_input(input), height, width, 999_999);
    Ok(universe.calculate_distances())
}

#[cfg(test)]
mod tests {
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
        assert_eq!(374, crate::part1::process(input)?);
        Ok(())
    }
}
