use crate::custom_error::AocError;
use crate::{extract_symbols, parse_digits};

fn sum_gears(input: &str) -> u32 {
    let numbers = parse_digits(input).unwrap();
    extract_symbols(input)
        .iter()
        .filter(|(c, _, _)| *c == '*')
        .map(|(_, line, col)| {
            numbers
                .iter()
                .filter(|number| number.is_adjacent(*line, *col))
                .map(|number| number.value)
                .collect::<Vec<_>>()
        })
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter().product::<u32>())
        .sum()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(sum_gears(input))
}
