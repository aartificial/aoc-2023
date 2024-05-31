use crate::custom_error::AocError;
use crate::part1::hash;
use std::borrow::Cow;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let mut map: HashMap<usize, Box> = HashMap::new();
    for label in input.split(',') {
        apply_operation(&mut map, label);
    }
    Ok(focusing_power(&map))
}

#[tracing::instrument(skip(map, label))]
fn apply_operation(map: &mut HashMap<usize, Box>, label: &str) {
    let label = Label::from(label);
    match label.operation {
        Operation::Equals(code, focal_length) => {
            let bx = map.entry(label.hash).or_insert_with(Box::default);
            bx.add(&code, focal_length);
        }
        Operation::Dash(code) => {
            if let Some(bx) = map.get_mut(&label.hash) {
                bx.remove(&code);
                if bx.lenses.is_empty() {
                    map.remove(&label.hash);
                }
            }
        }
    }
}

#[tracing::instrument(skip(map))]
fn focusing_power(map: &HashMap<usize, Box>) -> usize {
    map.iter()
        .flat_map(|(i, bx)| {
            bx.lenses
                .iter()
                .enumerate()
                .map(|(s, (_, fl))| calculate_fp(*i, s, *fl))
        })
        .sum()
}

const fn calculate_fp(i: usize, s: usize, fl: usize) -> usize {
    (i + 1) * (s + 1) * fl
}

#[derive(Debug)]
struct Label<'a> {
    hash: usize,
    operation: Operation<'a>,
}

impl<'a> From<&'a str> for Label<'a> {
    fn from(input: &'a str) -> Self {
        let mut parts = input.split('=');
        let code = parts.next().unwrap().trim_end_matches('-');

        let operation = match parts.next() {
            Some(value) => Operation::Equals(Cow::Borrowed(code), value.trim().parse().unwrap()),
            None => Operation::Dash(Cow::Borrowed(code)),
        };

        Self {
            hash: hash(code),
            operation,
        }
    }
}

#[derive(Debug)]
enum Operation<'a> {
    Dash(Cow<'a, str>),
    Equals(Cow<'a, str>, usize),
}

#[derive(Debug, PartialEq)]
pub struct Box {
    lenses: Vec<(String, usize)>,
}

impl Box {
    fn find(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|(l, _)| l == label)
    }

    fn remove(&mut self, label: &str) {
        if let Some(index) = self.find(label) {
            self.lenses.remove(index);
        }
    }

    fn add(&mut self, label: &str, focal_length: usize) {
        if let Some(index) = self.find(label) {
            self.lenses[index] = (label.to_owned(), focal_length);
        } else {
            self.lenses.push((label.to_owned(), focal_length));
        }
    }

    fn new(lenses: Vec<(String, usize)>) -> Self {
        Self { lenses }
    }

    fn default() -> Self {
        Self { lenses: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::part1::hash;
    use rstest::rstest;

    #[rstest]
    #[case("rn", 0)]
    #[case("cm", 0)]
    #[case("qp", 1)]
    #[case("pc", 3)]
    #[case("ot", 3)]
    #[case("ab", 3)]
    fn hash_test(#[case] input: &str, #[case] expected: usize) {
        let result = hash(input);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case("rn=1", Box::default(), Box::new(Vec::from([("rn".to_owned(), 1)])))]
    #[case("qp=3", Box::default(), Box::new(Vec::from([("qp".to_owned(), 3)])))]
    #[case("cm=2", Box::default(), Box::new(Vec::from([("cm".to_owned(), 2)])))]
    #[case("pc=4", Box::default(), Box::new(Vec::from([("pc".to_owned(), 4)])))]
    #[case("ot=9", Box::default(), Box::new(Vec::from([("ot".to_owned(), 9)])))]
    #[case("ab=5", Box::default(), Box::new(Vec::from([("ab".to_owned(), 5)])))]
    #[case("pc=6", Box::default(), Box::new(Vec::from([("pc".to_owned(), 6)])))]
    #[case("ot=7", Box::default(), Box::new(Vec::from([("ot".to_owned(), 7)])))]
    fn add_lens_to_box(#[case] label: &'static str, #[case] mut input: Box, #[case] expected: Box) {
        let label = Label::from(label);

        match label.operation {
            Operation::Equals(label, focal_length) => {
                input.add(&label, focal_length);
            }
            _ => unreachable!(),
        }

        assert_eq!(expected, input);
    }

    #[rstest]
    #[case("rn-", Box::default(), Box::default())]
    #[case("rn-", Box::new(Vec::from([("rn".to_owned(), 1)])), Box::default())]
    #[case("cm-", Box::new(Vec::from([("rn".to_owned(), 1)])), Box::new(Vec::from([("rn".to_owned(), 1)])))]
    #[case("cm-", Box::new(Vec::from([("rn".to_owned(), 1), ("cm".to_owned(), 2)])), Box::new(Vec::from([("rn".to_owned(), 1)])))]
    fn remove_label_from_box(#[case] label: &str, #[case] input: Box, #[case] expected: Box) {
        let mut result = input;
        let label = Label::from(label);
        match label.operation {
            Operation::Dash(code) => {
                println!("code: {}", code);
                result.remove(&code);
            }
            _ => unreachable!(),
        }

        assert_eq!(expected, result);
    }

    #[test]
    fn sequence() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let mut map: HashMap<usize, Box> = HashMap::new();

        for label in input.split(',') {
            apply_operation(&mut map, label);
        }

        assert_eq!(
            map,
            HashMap::from([
                (
                    0,
                    Box::new(Vec::from([("rn".to_owned(), 1), ("cm".to_owned(), 2)])),
                ),
                (
                    3,
                    Box::new(Vec::from([
                        ("ot".to_owned(), 7),
                        ("ab".to_owned(), 5),
                        ("pc".to_owned(), 6),
                    ])),
                ),
            ])
        );
    }

    #[test]
    fn focusing_power_test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let mut map: HashMap<usize, Box> = HashMap::new();
        for label in input.split(',') {
            apply_operation(&mut map, label);
        }

        assert_eq!(focusing_power(&map), 145);
    }
}
