use crate::custom_error::AocError;
use crate::number::Number;
use crate::{extract_symbols, parse_digits};
use itertools::Itertools;

pub fn collect_adjacents(input: &str, symbols: Vec<(char, usize, usize)>) -> Vec<Number> {
    parse_digits(input)
        .unwrap()
        .iter()
        .filter(|number| {
            symbols
                .iter()
                .any(|(_, line, col)| number.is_adjacent(*line, *col))
        })
        .copied()
        .collect_vec()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let x = collect_adjacents(input, extract_symbols(input))
        .iter()
        .fold(0u32, |acc, number| acc + number.value);
    Ok(x)
}
