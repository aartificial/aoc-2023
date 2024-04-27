use crate::custom_error::AocError;
use crate::part1::parse_input;
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    let lines = input.lines().map(|s| s.to_string()).collect_vec();
    let mut maze = parse_input(&lines);
    maze.walk_loop();
    Ok(maze.count_enclosed())
}

#[cfg(test)]
mod tests {
    use crate::part2::process;
    use tracing_subscriber::fmt;

    #[test]
    fn test_process() -> miette::Result<()> {
        fmt::init();
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        fmt::init();
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process3() -> miette::Result<()> {
        fmt::init();
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4, process(input)?);
        Ok(())
    }
}
