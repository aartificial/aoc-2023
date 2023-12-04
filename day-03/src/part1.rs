use crate::custom_error::AocError;
use crate::number::Number;
use nom::character::complete::{digit1, multispace1};
use nom::multi::separated_list0;
use nom::IResult;
use regex::Regex;

pub fn parsing_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(multispace1, digit1)(input)
}

fn number_from_match(match_indices: regex::Match, line_index: usize) -> Result<Number, String> {
    let start = match_indices.start();
    let end = match_indices.end();
    match match_indices.as_str().parse::<usize>() {
        Ok(value) => Ok(Number {
            value: value as u32,
            line: line_index,
            start,
            end: end - 1,
        }),
        Err(_) => Err(format!(
            "Failed to parse number: {}",
            match_indices.as_str()
        )),
    }
}

pub fn parse_digits(input: &str) -> Result<Vec<Number>, String> {
    let mut output = Vec::new();
    let regex = Regex::new(r"\d+").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    for (line_index, line) in lines.iter().enumerate() {
        for match_indices in regex.find_iter(line) {
            match number_from_match(match_indices, line_index) {
                Ok(num) => output.push(num),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(output)
}

pub fn collect_adjacents(input: &str) -> Vec<Number> {
    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(line, line_str)| {
            line_str
                .chars()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
                .map(move |(col, c)| (c, line, col))
        })
        .collect::<Vec<_>>();

    let parsed_digits = parse_digits(input).expect("Failed to parse digits");

    let adjacents = parsed_digits
        .iter()
        .filter(|number| {
            symbols
                .iter()
                .any(|(_, line, col)| number.is_adjacent(*line, *col))
        })
        .copied()
        .collect::<Vec<_>>();

    adjacents
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let x = collect_adjacents(input)
        .iter()
        .fold(0u32, |acc, number| acc + number.value);
    Ok(x)
}
