use itertools::Itertools;
use std::str::FromStr;

pub trait Reflection {
    fn is_perfect_reflection(pattern: &[String], coords: (usize, usize)) -> bool;
    fn find_reflection(pattern: &[String]) -> usize;
}

pub struct Notes<T>
where
    T: Reflection,
{
    pub patterns: Vec<T>,
}

impl<T> FromStr for Notes<T>
where
    T: Reflection + FromStr<Err = &'static str>,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            patterns: s
                .split("\n\n")
                .map(|pattern| pattern.parse().unwrap())
                .collect_vec(),
        })
    }
}
