use crate::custom_error::AocError;
use crate::shared::Sequence;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    Ok(Sequence::parse(input)
        .iter()
        .map(|s| Sequence::process(s))
        .map(|v| v.iter().map(|diff| diff.last().unwrap()).sum::<i32>())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
