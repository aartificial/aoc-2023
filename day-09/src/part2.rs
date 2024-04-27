use crate::custom_error::AocError;
use crate::shared::Sequence;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    Ok(Sequence::parse(input)
        .iter()
        .map(|s| Sequence::process(s))
        .map(|v| {
            v.iter()
                .rev()
                .skip(1)
                .map(|diff| diff.first().unwrap())
                .fold(0, |acc, &x| -acc + x)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use tracing_subscriber::fmt;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        fmt::init();
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
