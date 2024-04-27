use crate::custom_error::AocError;
use itertools::Itertools;
use num::integer::lcm;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;

const START: char = 'A';
const END: char = 'Z';

fn is_start(key: &str) -> bool {
    key.chars().last().unwrap() == START
}

fn is_end(key: &&str) -> bool {
    key.chars().last().unwrap() != END
}

#[derive(Debug, Clone)]
struct Node<'a> {
    value: &'a str,
    left: String,
    right: String,
}

impl<'a> Node<'a> {
    fn new(value: &'a str, left: String, right: String) -> Self {
        Self { value, left, right }
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (sequence, rest) = input.split_once("\n\n").unwrap();
    let map = parse_input(rest);
    let lcm = map
        .iter()
        .filter(|(key, _)| is_start(key))
        .map(|(_, value)| value)
        .cloned()
        .collect_vec()
        .par_iter()
        .map(|mut node| {
            let mut inner_seq = sequence.chars().cycle();
            let mut count = 0;
            while is_end(&node.value) {
                count += 1;
                match inner_seq.next().unwrap() {
                    'L' => node = map.get(&node.left).unwrap(),
                    'R' => node = map.get(&node.right).unwrap(),
                    _ => unreachable!("Invalid input"),
                }
            }
            count
        })
        .reduce(|| 1, lcm);

    Ok(lcm)
}

fn parse_input(rest: &str) -> HashMap<String, Node> {
    let map: HashMap<String, Node> = rest
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let (left, right) = value.split_once(", ").unwrap();
            (
                key.to_string(),
                Node::new(key, left.replace('(', ""), right.replace(')', "")),
            )
        })
        .collect();
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt;

    #[test]
    fn test_process() -> miette::Result<()> {
        fmt::init();
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
