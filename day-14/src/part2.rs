use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize, AocError> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let _input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        //assert_eq!(64, process(input)?);
        Ok(())
    }
}
