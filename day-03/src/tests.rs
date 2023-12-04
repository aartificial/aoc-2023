#[cfg(test)]
mod tests {
    use super::*;
    use crate::part1::{collect_adjacents, process};

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(8, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn sum_adjacents() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_digits() -> miette::Result<()> {
        let input = "1.3
.*.
4.5";
        assert_eq!(4, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn up_left() -> miette::Result<()> {
        let input = "1..
.*.
...";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn up_right() -> miette::Result<()> {
        let input = "..1
.*.
...";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn down_left() -> miette::Result<()> {
        let input = "...
.*.
1..";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn down_right() -> miette::Result<()> {
        let input = "...
.*.
..1";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn left() -> miette::Result<()> {
        let input = "...
1*.
...";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn right() -> miette::Result<()> {
        let input = "...
.*1
...";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn up() -> miette::Result<()> {
        let input = ".1.
.*.
...";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }

    #[test]
    fn down() -> miette::Result<()> {
        let input = "...
.*.
.1.";
        assert_eq!(1, collect_adjacents(input).len());
        Ok(())
    }
}
