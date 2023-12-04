use crate::custom_error::AocError;
use std::iter::from_fn;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let output = input.lines().map(parse_line).sum::<u32>();

    Ok(output)
}

pub fn parse_line(line: &str) -> u32 {
    let mut index = 0;
    let iter = from_fn(move || {
        let reduced = &line[index..];
        let result = if reduced.starts_with("one") {
            Some('1')
        } else if reduced.starts_with("two") {
            Some('2')
        } else if reduced.starts_with("three") {
            Some('3')
        } else if reduced.starts_with("four") {
            Some('4')
        } else if reduced.starts_with("five") {
            Some('5')
        } else if reduced.starts_with("six") {
            Some('6')
        } else if reduced.starts_with("seven") {
            Some('7')
        } else if reduced.starts_with("eight") {
            Some('8')
        } else if reduced.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced.chars().next();
            result
        };
        index += 1;
        result
    });
    let mut it = iter.filter_map(|c| c.to_digit(10));
    let first = it.next().expect("there should be a digit");
    match it.last() {
        Some(last) => format!("{first}{last}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
