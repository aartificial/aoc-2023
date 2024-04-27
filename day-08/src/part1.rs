use crate::custom_error::AocError;
use std::collections::HashMap;
use tracing::info;

const START: &str = "AAA";
const END: &str = "ZZZ";

#[derive(Debug)]
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

    let x: HashMap<String, Node> = rest
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let (left, right) = value.split_once(", ").unwrap();
            info!(%key, %left, %right);
            (
                key.to_string(),
                Node::new(key, left.replace('(', ""), right.replace(')', "")),
            )
        })
        .collect();

    let start = x.get(START).unwrap();
    let mut current = start;
    let mut sequence = sequence.chars().cycle();
    let mut count = 0;

    while current.value != END {
        count += 1;
        match sequence.next() {
            Some('L') => {
                info!("{} -> {}", current.value, current.left);
                current = x.get(&current.left).unwrap();
            }
            Some('R') => {
                info!("{} -> {}", current.value, current.right);
                current = x.get(&current.right).unwrap();
            }
            _ => {
                return Err(AocError::InvalidInput);
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
