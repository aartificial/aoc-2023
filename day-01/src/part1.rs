use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));
            let first = it.next().expect("there should be a digit");

            match it.last() {
                Some(last) => format!("{first}{last}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a valid number")
        })
        .sum::<u32>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
